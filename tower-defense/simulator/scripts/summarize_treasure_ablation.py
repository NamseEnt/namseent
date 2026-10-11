#!/usr/bin/env python3
"""Summarize paired terminal-eval JSON without treating progress as win rate."""
import argparse
from collections import Counter
import json
import math
from pathlib import Path
import random


def wilson_interval(picks, offers):
    if not offers:
        return None
    z = 1.96
    p = picks / offers
    denominator = 1 + z * z / offers
    center = (p + z * z / (2 * offers)) / denominator
    margin = z * math.sqrt(p * (1 - p) / offers + z * z / (4 * offers * offers)) / denominator
    return [max(0.0, center - margin), min(1.0, center + margin)]


def aggregate(episodes, treasure):
    offers = sum(e['treasure_offers'].get(treasure, 0) for e in episodes)
    picks = sum(e['treasure_selections'].get(treasure, 0) for e in episodes)
    first_offers = first_picks = 0
    probabilities = [d['option_probabilities'][treasure]
                     for e in episodes for d in e.get('treasure_decisions', [])
                     if treasure in d.get('option_probabilities', {})]
    for episode in episodes:
        decisions = episode.get('treasure_decisions', [])
        if decisions and treasure in decisions[0]['options']:
            first_offers += 1
            first_picks += decisions[0]['chosen'] == treasure
    return {
        'games': len(episodes), 'offers': offers, 'picks': picks,
        'mean_select_probability_given_offer': sum(probabilities) / len(probabilities) if probabilities else None,
        'probability_observations': len(probabilities),
        'pick_rate_given_offer': picks / offers if offers else None,
        'first_treasure_offers': first_offers, 'first_treasure_picks': first_picks,
        'first_treasure_pick_rate': first_picks / first_offers if first_offers else None,
        'first_treasure_wilson_ci95': wilson_interval(first_picks, first_offers),
        'mean_progress': sum(e['terminal_clear_rate'] for e in episodes) / len(episodes),
        'victories': sum(e['victory'] for e in episodes),
        'illegal_actions': sum(e['illegal_actions'] for e in episodes),
        'fallback_actions': sum(e['fallback_actions'] for e in episodes),
        'post_sampling_mutations': sum(e['post_sampling_mutations'] for e in episodes),
    }


def paired_rate_change(before, after, treasure):
    left = {e['seed']: e for e in before}
    right = {e['seed']: e for e in after}
    if left.keys() != right.keys():
        raise ValueError('paired comparison requires exactly the same game seeds')
    seeds = sorted(left)
    counts = [(left[s]['treasure_selections'].get(treasure, 0),
               left[s]['treasure_offers'].get(treasure, 0),
               right[s]['treasure_selections'].get(treasure, 0),
               right[s]['treasure_offers'].get(treasure, 0)) for s in seeds]

    def delta(rows):
        a, b, c, d = map(sum, zip(*rows))
        return c / d - a / b if b and d else None

    rng = random.Random(8342)
    boot = [delta(rng.choices(counts, k=len(counts))) for _ in range(2000)]
    boot = sorted(value for value in boot if value is not None)
    return {'after_minus_before': delta(counts),
            'paired_game_bootstrap_ci95': [boot[int(.025 * len(boot))], boot[min(len(boot)-1, int(.975 * len(boot)))]] if boot else None,
            'bootstrap_replicates': 2000, 'bootstrap_seed': 8342,
            'note': 'Game-cluster bootstrap; natural play can change later offer opportunities.'}


def summarize(baseline, original, disabled):
    picks = Counter()
    for e in baseline['episodes']:
        if e['policy'] == 'source':
            picks.update(e['treasure_selections'])
    if not picks:
        raise ValueError('baseline has no source treasure selections')
    treasure = sorted(picks, key=lambda key: (-picks[key], key))[0]
    result = {'schema_version': 1, 'target_treasure': treasure,
              'target_selection': 'Most source selections on the pre-training 128-game development probe',
              'baseline_selection_counts': dict(picks.most_common()),
              'conditions': {}, 'comparisons': {}}
    for name, report in [('original_effect', original), ('disabled_effect', disabled)]:
        groups = {policy: [e for e in report['episodes'] if e['policy'] == policy]
                  for policy in report['policies'] if policy != 'canonical'}
        result['conditions'][name] = {p: aggregate(g, treasure) for p, g in groups.items()}
        result['comparisons'][name] = {
            'retrained_vs_source': paired_rate_change(groups['source'], groups['retrained'], treasure),
            'retrained_vs_control': paired_rate_change(groups['control'], groups['retrained'], treasure),
        }
    result['limits'] = ['One paired training seed; not an independent-repeats result.',
                        'Final iteration at equal rollout budgets, not selected for treasure pick rate.',
                        'No proof of faster adaptation than training from scratch.',
                        'A lower pick rate alone does not establish a stronger game policy.',
                        'Probability diagnostics are conditional on choosing the SelectTreasure family.',
                        'Empirical bootstrap intervals can degenerate at unanimous choices; this is not proof of identical preferences.']
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline', type=Path, required=True)
    parser.add_argument('--original', type=Path, required=True)
    parser.add_argument('--disabled', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    read = lambda path: json.loads(path.read_text())
    result = summarize(read(args.baseline), read(args.original), read(args.disabled))
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result, indent=2))


if __name__ == '__main__':
    main()
