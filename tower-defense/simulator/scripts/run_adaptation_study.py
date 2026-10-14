#!/usr/bin/env python3
"""Reproduce the fixed-budget, three-repeat content adaptation study."""
import argparse
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
BINARY = ROOT / 'target/release/td-simulator'

def write(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')

def execute(study, name, arguments, threads):
    log = study / f'{name}.log'
    command = [str(BINARY), *arguments, '--threads', str(threads)]
    started = time.monotonic()
    with log.open('w') as output:
        process = subprocess.Popen(command, cwd=ROOT, stdout=output, stderr=subprocess.STDOUT)
        verified = False
        for _ in range(100):
            try:
                executable = Path(f'/proc/{process.pid}/exe').resolve(strict=True)
                if executable == BINARY.resolve():
                    command_line = Path(f'/proc/{process.pid}/cmdline').read_bytes().replace(b'\0', b' ').decode()
                    write(study / f'{name}.execution.json', {'pid':process.pid, 'executable':str(executable), 'command':command_line, 'release_sha256':hashlib.sha256(BINARY.read_bytes()).hexdigest()})
                    verified = True
                    break
            except FileNotFoundError:
                pass
            time.sleep(.01)
        if not verified:
            process.terminate()
            raise RuntimeError(f'{name}: could not verify release executable')
        checked_first_iteration = False
        while True:
            child, wait_status, usage = os.wait4(process.pid, os.WNOHANG)
            if child:
                break
            metadata_path = study / name / 'ppo.json'
            if not checked_first_iteration and metadata_path.exists():
                metadata = json.loads(metadata_path.read_text())
                first = next((row for row in metadata['history'] if row['iteration']==1), None)
                if first:
                    checked_first_iteration = True
                    print(f"{name} first iteration: rollout {first['rollout_seconds']:.1f}s, update {first['update_seconds']:.1f}s", flush=True)
                    if first['update_seconds'] > 180 or first['rollout_seconds'] > 180:
                        process.terminate()
                        process.wait()
                        raise RuntimeError(f'{name}: unexpectedly slow; stopped at complete checkpoint')
            time.sleep(.5)
        status = os.waitstatus_to_exitcode(wait_status)
        process.returncode = status
        write(study / f'{name}.resources.json', {'wall_seconds':time.monotonic()-started,'peak_rss_kib':usage.ru_maxrss,'user_seconds':usage.ru_utime,'system_seconds':usage.ru_stime})
    if status:
        raise RuntimeError(f'{name} exited {status}: {log.read_text()[-2000:]}')
    print(f'completed {name}', flush=True)


def plan(study, pilot):
    if (study / 'protocol.json').exists():
        raise ValueError('Protocol already exists; use train/evaluate to resume it')
    template = json.loads(pilot.read_text())
    study.mkdir(parents=True, exist_ok=True)
    template['source_checkpoint'] = str((pilot.parent/template['source_checkpoint']).resolve())
    for key, name in [('source_config','original.jsonc'),('target_config','disabled.jsonc')]:
        path = study/name
        if not path.exists():
            path.write_bytes((pilot.parent/template[key]).read_bytes())
        template[key]=name
    # Numeric target differs only in the authoritative starting-gold parameter.
    original = (study / 'original.jsonc').read_text()
    (study / 'gold105.jsonc').write_text(original.replace('"starting_gold": 100', '"starting_gold": 105', 1))
    jobs = []
    for repeat in range(3):
        for index, (condition, initialization) in enumerate([
            ('numeric','reuse_weights'), ('numeric','scratch'),
            ('effect','reuse_weights'), ('effect','scratch'), ('control','reuse_weights')]):
            name = f'{condition}-{initialization}-r{repeat+1}'
            spec = json.loads(json.dumps(template))
            spec.update(run_dir=name, iterations=4, seed=1101+repeat, train_seed_start=6400000+repeat*10000,
                        target_config={'numeric':'gold105.jsonc','effect':'disabled.jsonc','control':'original.jsonc'}[condition],
                        effect_inputs=condition!='numeric', initialization=initialization)
            spec['ppo'].update(seed=spec['seed'],train_seed_start=spec['train_seed_start'],episodes_per_iteration=16,policy_temperature=1.0)
            if condition == 'numeric':
                spec.pop('content_teacher', None)
            else:
                spec['content_teacher']['seed_start'] = 6500000+repeat*10000
            write(study / f'{name}.json', spec)
            jobs.append({'name':name,'condition':condition,'initialization':initialization,'repeat':repeat+1})
    baseline = json.loads(json.dumps(template))
    baseline.pop('content_teacher', None)
    baseline.update(run_dir='source-converted',iterations=1,effect_inputs=False,train_seed_start=6700000,seed=3301,target_config='original.jsonc')
    baseline['ppo'].update(seed=3301,train_seed_start=6700000,episodes_per_iteration=1,update_epochs=1,actor_learning_rate=1e-10,critic_learning_rate=1e-10,policy_temperature=1.0)
    write(study/'source-converted.json',baseline)
    write(study/'protocol.json', {
        'schema_version':1,'locked_at_utc':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()),
        'primary_metric':'terminal progress on a 0-100 scale; victories reported separately',
        'development_goal':40.0,'goal_rule':'first completed initialization/iteration with mean development progress >=40; no success assigned to censored runs',
        'checkpoint_selection':'highest mean development progress, later iteration breaks ties',
        'repeats':3,'jobs':jobs,'ppo_episodes_per_run':64,'teacher_budget':'same 16 paired source/target games and four resampled hidden-order scenarios per candidate in both effect arms; all simulation costs counted separately',
        'development_seeds':[4000000,4000031],'final_seeds':[7000000,7000127],
        'training_seeds':'paired within repeat and distinct between repeats; teacher seeds disjoint from PPO and final',
        'final_policy_rule':'protocol fixed before any final seeds; do not select checkpoint, recipe or treasure by final results',
        'cost_scope':'post-change preparation, teacher, PPO and development evaluation; source pretraining is recorded separately as a sunk cost',
        'manual_preparation':'one shared implementation and protocol preparation; no run-specific hand labels or treasure ID overrides',
        'release_sha256':hashlib.sha256(BINARY.read_bytes()).hexdigest(),
        'limits':['fixed finite budget; failure to reach the target is right censored, not an exact speedup factor','three independent training seeds; held-out uncertainty does not replace between-training variation','matched random seeds do not imply identical state trajectories after actions diverge']})
    print(f'locked {len(jobs)} runs in {study / "protocol.json"}')


def train(study, workers, threads):
    protocol = json.loads((study/'protocol.json').read_text())
    if hashlib.sha256(BINARY.read_bytes()).hexdigest() != protocol['release_sha256']:
        raise ValueError('Release binary differs from locked protocol')
    jobs = [{'name':'source-converted'},*protocol['jobs']]
    def run(job):
        name=job['name']
        if (study/name/'retraining-report.json').exists():
            return
        execute(study,name,['ml','phase4','retrain','--spec',str(study/f'{name}.json')],threads)
        metadata=json.loads((study/name/'ppo.json').read_text())
        first=next(row for row in metadata['history'] if row['iteration']==1)
        if first['update_seconds']>180 or first['rollout_seconds']>180:
            raise RuntimeError(f'{name}: unexpectedly slow first iteration; inspect recorded timings')
    with ThreadPoolExecutor(max_workers=workers) as executor:
        list(executor.map(run,jobs))


def evaluate(study, threads):
    protocol=json.loads((study/'protocol.json').read_text())
    for condition, config in [('original','original.jsonc'),('disabled','disabled.jsonc'),('numeric','gold105.jsonc')]:
        output=study/f'final-{condition}.json'
        if output.exists():
            raise ValueError(f'{output} already exists; final evaluation cannot be silently repeated')
        arguments=['ml','phase4','terminal-eval','--split','effect-adaptation-validation','--count','128','--config',str(study/config),'--output',str(output),'--confirm-final','--allow-checkpoint-config-change', '--policy', f'source={study}/source-converted/iter-0000']
        jobs=[job for job in protocol['jobs'] if (job['condition']=='numeric')==(condition=='numeric')]
        for job in jobs:
            report=json.loads((study/job['name']/'retraining-report.json').read_text())
            arguments += ['--policy', f"{job['name']}={study/job['name']}/iter-{report['selected_iteration']:04}"]
        for job in jobs:
            if job['initialization']=='reuse_weights' and job['condition']!='control':
                scratch=job['name'].replace('reuse_weights','scratch')
                arguments += ['--compare',f"{job['name']}:{scratch}"]
            if job['condition']=='effect' and job['initialization']=='reuse_weights':
                arguments += ['--compare',f"{job['name']}:control-reuse_weights-r{job['repeat']}"]
        execute(study,f'final-{condition}',arguments,threads)


def account(study):
    protocol = json.loads((study/'protocol.json').read_text())
    example = ROOT/'target/release/examples/replay_content_teacher_cost'
    jobs = [job for job in protocol['jobs'] if job['condition'] != 'numeric' and not (study/job['name']/'content-teacher-collection-cost.json').exists()]
    subprocess.run([str(example),*[str(study/job['name']) for job in jobs]],cwd=ROOT,check=True)


def benchmark(study, threads):
    protocol=json.loads((study/'protocol.json').read_text())
    for job in protocol['jobs']:
        name=job['name']
        report=json.loads((study/name/'retraining-report.json').read_text())
        config={'numeric':'gold105.jsonc','effect':'disabled.jsonc','control':'original.jsonc'}[job['condition']]
        execute(study,f'inference-{name}', ['simulate','--checkpoint',f"{study/name}/iter-{report['selected_iteration']:04}",'--config',str(study/config),'--samples','4','--seed-start','6108000','--db',str(study/f'inference-{name}.db'),'--quiet'],threads)


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action',choices=['plan','train','evaluate','account','benchmark'])
    parser.add_argument('--study',type=Path,required=True)
    parser.add_argument('--pilot',type=Path)
    parser.add_argument('--workers',type=int,default=2)
    parser.add_argument('--threads',type=int,default=5)
    args=parser.parse_args()
    study=args.study.resolve()
    if args.action=='plan': plan(study,args.pilot.resolve())
    elif args.action=='train': train(study,args.workers,args.threads)
    elif args.action=='evaluate': evaluate(study,args.threads)
    elif args.action=='account': account(study)
    else: benchmark(study,args.threads)

if __name__=='__main__': main()
