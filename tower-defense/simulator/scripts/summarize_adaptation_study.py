#!/usr/bin/env python3
"""Summarize frozen adaptation repeats, costs, and untouched paired evaluations."""
import argparse
import hashlib
import json
from pathlib import Path
import random
import statistics
from summarize_treasure_ablation import aggregate, paired_rate_change


def read(path):
    return json.loads(path.read_text())


def interval(values):
    values=sorted(values)
    return [values[int(.025*len(values))],values[min(len(values)-1,int(.975*len(values)))]]


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--study',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args(); study=args.study.resolve();protocol=read(study/'protocol.json')
    result={'schema_version':1,'study_directory':str(study),'protocol':protocol,'runs':{},'conditions':{},'efficiency':{},'limits':[
        'One fixed PPO recipe and 64 direct rollout games per run; scratch results are budget censored.',
        'Past source PPO is a separate sunk cost; its historical BC/teacher cost is unavailable, so no end-to-end training-cost claim is made.',
        'Teacher simulation and semantic decisions are included; its scenario rollouts are distinct from complete actual games.',
        'Three independent training repeats cannot establish a general algorithmic speedup.',
        'Natural-play later treasure opportunities depend on previous policy actions; first-selection diagnostics are also reported.',
        'Treasure option probabilities are conditional on the SelectTreasure family; actual picks given an offer are reported separately.',
        'The selected checkpoint maximizes development progress, never treasure suppression.',
        'Inference timing includes normal operating-system scheduling under the reported thread count; process peak memory is not an isolated model allocation.'
    ]}
    result['shared_accounting_overhead']={'source_conversion_process':read(study/'source-converted.resources.json'),
        'note':'baseline conversion and accounting passes are shared study preparation, outside per-arm learning budgets'}
    prototype=study.parent/'effect-adaptation-20261011'
    if prototype != study and (prototype/'development-diagnostic.json').exists():
        old_reports=[read(path) for path in prototype.glob('*-r*/retraining-report.json')]
        result['development_prototypes']={
            'directory':str(prototype),'run_count':len(old_reports),'final_seeds_used':False,
            'ppo_actual_games':sum(r['post_change_budget']['episodes'] for r in old_reports),
            'teacher_actual_games':sum((r.get('content_teacher') or {}).get('games',0) for r in old_reports),
            'teacher_scenario_rollouts':sum((r.get('content_teacher') or {}).get('scenario_rollouts',0) for r in old_reports),
            'summed_process_wall_seconds':sum(read(prototype/f"{path.parent.name}.resources.json")['wall_seconds'] for path in prototype.glob('*-r*/retraining-report.json')),
            'reason':'shared candidate projection allowed treasure preparation to alter unrelated action families; replaced by isolated treasure projection before final'}
    result['development_pilots']=[]
    for name in ['pilot-temperature3','pilot-teacher']:
        path=prototype/name/'retraining-report.json'
        if path.exists():
            report=read(path);teacher=report.get('content_teacher') or {}
            result['development_pilots'].append({'name':name,'ppo_budget':report['post_change_budget'],'teacher_actual_games':teacher.get('games',0),'teacher_scenario_rollouts':teacher.get('scenario_rollouts',0),'teacher_seconds':teacher.get('seconds',0),'scope':'development only; recorded learning costs; report-only resume overhead excluded'})
    for job in protocol['jobs']:
        name=job['name'];report=read(study/name/'retraining-report.json');metadata=read(study/name/'ppo.json');teacher=report.get('content_teacher') or {}
        first=None
        for row in metadata['history']:
            scores=[s for s in (row.get('evaluation') or {}).get('summaries',[]) if s['policy']=='ppo']
            if scores and scores[0]['mean_terminal_clear_rate']>=protocol['development_goal']:
                first={'iteration':row['iteration'],'development_progress':scores[0]['mean_terminal_clear_rate'],
                       'initial_development_seconds':metadata['history'][0]['evaluation']['wall_seconds'],
                       'ppo_budget':row['cumulative'],'teacher_seconds':teacher.get('seconds',0),
                       'teacher_actual_games':teacher.get('games',0),'teacher_scenario_rollouts':teacher.get('scenario_rollouts',0),'teacher_scenario_decisions':teacher.get('scenario_decisions',0),
                       'preparation_seconds':report['preparation_seconds']}
                break
        result['runs'][name]={**job,'selected_iteration':report['selected_iteration'],'ppo_recipe':metadata['config'],'source_budget':report['source_budget'],'source_weights_reused':job['initialization']=='reuse_weights',
            'post_change_ppo_budget':report['post_change_budget'],'content_teacher':teacher,'teacher_collection_accounting':read(study/name/'content-teacher-collection-cost.json') if teacher else None,
            'inference_process_resources':read(study/f'inference-{name}.resources.json'),
            'preparation_seconds':report['preparation_seconds'],
            'workflow_invocation_seconds':report['workflow_invocation_seconds'],'resources':read(study/f'{name}.resources.json'),
            'first_development_goal':first,'goal_right_censored':first is None}
    for run in result['runs'].values():
        teacher=run['content_teacher']; accounting=run['teacher_collection_accounting'] or {}
        actual=accounting.get('actual_collection_semantic_decisions',0)
        run['post_change_total_actual_training_games']=run['post_change_ppo_budget']['episodes']+teacher.get('games',0)
        run['post_change_total_training_semantic_decisions']=run['post_change_ppo_budget']['decisions']+teacher.get('scenario_decisions',0)+actual
        goal=run['first_development_goal']
        if goal:
            goal['total_actual_training_games']=goal['ppo_budget']['episodes']+teacher.get('games',0)
            goal['total_training_semantic_decisions']=goal['ppo_budget']['decisions']+teacher.get('scenario_decisions',0)+actual
            goal['accounted_preparation_training_development_seconds']=goal['preparation_seconds']+goal['teacher_seconds']+goal['initial_development_seconds']+goal['ppo_budget']['seconds']
    reports={c:read(study/f'final-{c}.json') for c in ['original','disabled','numeric']}
    result['resume_verification']=read(study/'resume-verification.json') if (study/'resume-verification.json').exists() else None
    result['environment_contracts']={c:{k:v for k,v in report['environment'].items() if k not in ['game_config','content_catalog']} for c,report in reports.items()}
    for condition,report in reports.items():
        if report['seeds'] != list(range(7000000,7000128)): raise ValueError('unexpected final seeds')
        groups={p:[e for e in report['episodes'] if e['policy']==p] for p in report['policies']}
        result['conditions'][condition]={'policies':{},'paired_progress':report['comparisons'],'wall_seconds':report['wall_seconds'],'resources':read(study/f'final-{condition}.resources.json')}
        for policy,episodes in groups.items():
            summary=next(s for s in report['summaries'] if s['policy']==policy)
            result['conditions'][condition]['policies'][policy]={**aggregate(episodes,'black_white'),
                'mean_decision_ms':summary['mean_decision_ms'],'mean_forward_ms':summary['mean_forward_ms'],
                'decisions_per_second':summary['decisions_per_second']}
    for condition in ['numeric','effect']:
        final=reports['numeric' if condition=='numeric' else 'disabled']
        names=[f'{condition}-reuse_weights-r{i}' for i in range(1,4)]
        scratch=[n.replace('reuse_weights','scratch') for n in names]
        delta_rows=[]
        for left,right in zip(names,scratch):
            a={e['seed']:e['terminal_clear_rate'] for e in final['episodes'] if e['policy']==left}
            b={e['seed']:e['terminal_clear_rate'] for e in final['episodes'] if e['policy']==right}
            if a.keys()!=b.keys(): raise ValueError('unpaired final comparison')
            delta_rows.append([a[s]-b[s] for s in sorted(a)])
        rng=random.Random(20431)
        # Resample training repeats, then shared held-out seed clusters. This retains pairing.
        bootstrap=[]
        for _ in range(4000):
            repeats=rng.choices(range(3),k=3);seeds=rng.choices(range(128),k=128)
            bootstrap.append(sum(delta_rows[r][s] for r in repeats for s in seeds)/(3*128))
        achieved=[result['runs'][n]['first_development_goal'] for n in names]
        scratch_achieved=[result['runs'][n]['first_development_goal'] for n in scratch]
        result['efficiency'][condition]={
            'retrained_development_goal_reached':sum(x is not None for x in achieved),
            'retrained_accounted_time_to_goal_seconds':[x['accounted_preparation_training_development_seconds'] if x else None for x in achieved],
            'scratch_accounted_budget_seconds':[result['runs'][n]['preparation_seconds']+result['runs'][n]['content_teacher'].get('seconds',0)+result['runs'][n]['post_change_ppo_budget']['seconds']+read(study/n/'ppo.json')['history'][0]['evaluation']['wall_seconds'] for n in scratch],
            'scratch_development_goal_reached':sum(x is not None for x in scratch_achieved),
            'retrained_final_progress':[result['conditions']['numeric' if condition=='numeric' else 'disabled']['policies'][n]['mean_progress'] for n in names],
            'scratch_final_progress':[result['conditions']['numeric' if condition=='numeric' else 'disabled']['policies'][n]['mean_progress'] for n in scratch],
            'mean_paired_progress_advantage':statistics.mean(statistics.mean(v) for v in delta_rows),
            'hierarchical_repeat_and_seed_bootstrap_ci95':interval(bootstrap),
            'bootstrap_seed':20431,'bootstrap_replicates':4000,
            'exact_time_speedup_factor':None,
            'conclusion':'Report goal attainment and finite-budget progress; scratch censoring precludes an exact time-to-goal ratio.'}
    original=reports['original'];disabled=reports['disabled']
    result['treasure_adaptation']={}
    for i in range(1,4):
        n=f'effect-reuse_weights-r{i}';c=f'control-reuse_weights-r{i}'
        groups=lambda report,p:[e for e in report['episodes'] if e['policy']==p]
        a=groups(original,n);b=groups(disabled,n)
        first_a={e['seed']:e['treasure_decisions'][0] for e in a if e['treasure_decisions'] and 'black_white' in e['treasure_decisions'][0]['options']}
        first_b={e['seed']:e['treasure_decisions'][0] for e in b if e['treasure_decisions'] and 'black_white' in e['treasure_decisions'][0]['options']}
        paired=sorted(seed for seed in first_a.keys() & first_b.keys() if first_a[seed]['options']==first_b[seed]['options'] and first_a[seed]['stage']==first_b[seed]['stage'])
        probability_deltas=[first_b[s]['option_probabilities']['black_white']-first_a[s]['option_probabilities']['black_white'] for s in paired]
        rng=random.Random(9342)
        probability_bootstrap=[statistics.mean(rng.choices(probability_deltas,k=len(probability_deltas))) for _ in range(2000)] if probability_deltas else []
        result['treasure_adaptation'][n]={
            'disabled_retrained_vs_source':paired_rate_change(groups(disabled,'source'),b,'black_white'),
            'disabled_retrained_vs_enabled_training_control':paired_rate_change(groups(disabled,c),b,'black_white'),
            'same_model_disabled_vs_original':paired_rate_change(a,b,'black_white'),
            'matched_first_offer_games':len(paired),
            'matched_first_offer_probability_delta':statistics.mean(probability_deltas) if probability_deltas else None,
            'matched_first_offer_probability_delta_bootstrap_ci95':interval(probability_bootstrap) if probability_bootstrap else None,
            'matched_first_offer_choices_changed':sum(first_a[s]['chosen']!=first_b[s]['chosen'] for s in paired)}
    result['artifact_hashes']={str(path.relative_to(study)):hashlib.sha256(path.read_bytes()).hexdigest()
        for path in [study/'protocol.json',*[study/f'final-{c}.json' for c in reports],*[study/job['name']/f"iter-{result['runs'][job['name']]['selected_iteration']:04}"/'actor.bin' for job in protocol['jobs']]]}
    for job in protocol['jobs']:
        name=job['name']; checkpoint=study/name/f"iter-{result['runs'][name]['selected_iteration']:04}"
        for path in [checkpoint/'critic.bin',checkpoint/'ppo-actor.json',checkpoint/'actor-optimizer.bin',checkpoint/'critic-optimizer.bin',study/name/'ppo.json']:
            result['artifact_hashes'][str(path.relative_to(study))]=hashlib.sha256(path.read_bytes()).hexdigest()
    args.output.parent.mkdir(parents=True,exist_ok=True)
    args.output.write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({'efficiency':result['efficiency'],'treasure_adaptation':result['treasure_adaptation']},indent=2))

if __name__=='__main__': main()
