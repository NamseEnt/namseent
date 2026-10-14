#!/usr/bin/env python3
"""Summarize paired Core construction-context development artifacts."""

from __future__ import annotations

import argparse
import json
import math
from pathlib import Path


T_95_DF3 = 3.182446


def option_key(estimate: dict) -> tuple[tuple[int, ...], int] | None:
    action = estimate.get("action", {}).get("BuildTower")
    if action is None:
        return None
    return tuple(sorted(action["card_ids"])), action["hand_slot_index"]


def option_id(key: tuple[tuple[int, ...], int]) -> str:
    cards, slot = key
    return f"cards={list(cards)};slot={slot}"


def estimate_map(decision: dict) -> dict[tuple[tuple[int, ...], int], dict]:
    return {
        key: candidate
        for candidate in decision["candidates"]
        if (key := option_key(candidate)) is not None
    }


def terminal_progress(candidate: dict) -> float:
    """Remove the teacher's 1,000-point terminal victory bonus."""
    return candidate["mean_score"] - 1_000.0 * candidate["wins"] / candidate["sample_count"]


def metadata_map(data: dict, side: str) -> dict[tuple[tuple[int, ...], int], dict]:
    field = f"{side}_build_option_info"
    result = {}
    for row in data[field]:
        if row is not None:
            key = tuple(sorted(row["card_ids"])), row["hand_slot_index"]
            result[key] = row
    return result


def tower_map(data: dict, side: str) -> dict[tuple[int, ...], dict]:
    observation_key = {
        "plain": "plain_observation",
        "treasure": "treasure_observation",
        "baseline": "baseline_observation",
        "changed": "changed_observation",
    }[side]
    observation = data[observation_key]
    return {
        tuple(sorted(row["card_ids"])): row["template"]
        for row in observation["build_tower_candidates"]
    }


def sample_summary(values: list[float]) -> dict:
    n = len(values)
    mean = sum(values) / n if n else None
    if n < 2:
        return {"n": n, "mean": mean, "sd": None, "se": None, "ci95_t_df3": None}
    variance = sum((value - mean) ** 2 for value in values) / (n - 1)
    sd = math.sqrt(variance)
    se = sd / math.sqrt(n)
    return {
        "n": n,
        "mean": mean,
        "sd": sd,
        "se": se,
        "ci95_t_df3": [mean - T_95_DF3 * se, mean + T_95_DF3 * se] if n == 4 else None,
    }


def summarize_pair(path: Path, lhs: str, rhs: str, condition: str) -> dict:
    data = json.loads(path.read_text())
    scenarios = data["paired_scenario_values"]
    left_rows = [estimate_map(scenario[lhs]) for scenario in scenarios]
    right_rows = [estimate_map(scenario[rhs]) for scenario in scenarios]
    common = sorted(set.intersection(*(set(row) for row in left_rows + right_rows)))
    metadata_left = metadata_map(data, lhs)
    metadata_right = metadata_map(data, rhs)
    left_towers = tower_map(data, lhs)
    right_towers = tower_map(data, rhs)
    candidates = []
    for key in common:
        left_details = [row[key] for row in left_rows]
        right_details = [row[key] for row in right_rows]
        left_values = [terminal_progress(row) for row in left_details]
        right_values = [terminal_progress(row) for row in right_details]
        deltas = [right - left for left, right in zip(left_values, right_values)]
        candidates.append(
            {
                "candidate": option_id(key),
                "card_ids": list(key[0]),
                "tower_slot": key[1],
                "outside_top8": metadata_left.get(key, {}).get("outside_v1_top8"),
                "left_terminal_progress": sample_summary(left_values),
                "right_terminal_progress": sample_summary(right_values),
                "terminal_progress_right_minus_left_paired": sample_summary(deltas),
                "left_clear_rate_mean": sum(row["mean_clear_rate"] for row in left_details) / len(left_details),
                "right_clear_rate_mean": sum(row["mean_clear_rate"] for row in right_details) / len(right_details),
                "left_final_stage_mean": sum(row["mean_final_stage"] for row in left_details) / len(left_details),
                "right_final_stage_mean": sum(row["mean_final_stage"] for row in right_details) / len(right_details),
                "left_result_tower": left_towers.get(key[0]),
                "right_result_tower": right_towers.get(key[0]),
            }
        )

    crossovers = []
    pair_differences = []
    for index, first in enumerate(common):
        for second in common[index + 1 :]:
            left_delta = [terminal_progress(row[first]) - terminal_progress(row[second]) for row in left_rows]
            right_delta = [terminal_progress(row[first]) - terminal_progress(row[second]) for row in right_rows]
            paired_context_delta = [r - l for l, r in zip(left_delta, right_delta)]
            left_mean = sum(left_delta) / len(left_delta)
            right_mean = sum(right_delta) / len(right_delta)
            record = {
                "first": option_id(first),
                "second": option_id(second),
                "left_first_minus_second": sample_summary(left_delta),
                "right_first_minus_second": sample_summary(right_delta),
                "paired_context_change": sample_summary(paired_context_delta),
                "rank_reversal": left_mean * right_mean < 0,
            }
            pair_differences.append(record)
            if record["rank_reversal"]:
                crossovers.append(record)

    invariants = {
        "same_hand": data["plain_observation"]["hand"] == data["treasure_observation"]["hand"]
        if lhs == "plain"
        else data["baseline_observation"]["hand"] == data["changed_observation"]["hand"],
        "same_map": data["plain_observation"]["route_coords"] == data["treasure_observation"]["route_coords"]
        if lhs == "plain"
        else data["baseline_observation"]["route_coords"] == data["changed_observation"]["route_coords"],
        "same_towers": data["plain_observation"]["tower_grid"] == data["treasure_observation"]["tower_grid"]
        if lhs == "plain"
        else data["baseline_observation"]["tower_grid"] == data["changed_observation"]["tower_grid"],
        "same_gold_hp": (
            data["plain_observation"]["gold"] == data["treasure_observation"]["gold"]
            and data["plain_observation"]["hp_raw"] == data["treasure_observation"]["hp_raw"]
        )
        if lhs == "plain"
        else (
            data["baseline_observation"]["gold"] == data["changed_observation"]["gold"]
            and data["baseline_observation"]["hp_raw"] == data["changed_observation"]["hp_raw"]
        ),
    }
    return {
        "condition": condition,
        "source_artifact": path.name,
        "state_seed": data["state_seed"],
        "scenario_seeds": [scenario["scenario_seed"] for scenario in scenarios],
        "state_hashes": {
            lhs: data["plain_state_hash"] if lhs == "plain" else data["baseline_state_hash"],
            rhs: data["treasure_state_hash"] if rhs == "treasure" else data["changed_state_hash"],
        },
        "invariants": invariants,
        "candidate_count": len(common),
        "continuation_count": data["actual_candidate_scenario_rollouts"],
        "candidate_values": candidates,
        "pairwise_candidate_values": pair_differences,
        "rank_reversals": crossovers,
    }


def summarize_confirmation(path: Path, fixture_path: Path, lhs: str, rhs: str) -> dict:
    data = json.loads(path.read_text())
    fixture = json.loads(fixture_path.read_text())
    left = estimate_map(data[lhs])
    right = estimate_map(data[rhs])
    common = sorted(set(left) & set(right))
    left_meta = metadata_map(fixture, "plain" if lhs == "plain_values" else "baseline")
    right_meta = metadata_map(fixture, "treasure" if rhs == "treasure_values" else "changed")
    candidates = []
    for key in common:
        l, r = left[key], right[key]
        candidates.append(
            {
                "candidate": option_id(key),
                "card_ids": list(key[0]),
                "tower_slot": key[1],
                "outside_top8": left_meta.get(key, {}).get("outside_v1_top8"),
                "left_terminal_progress_mean": terminal_progress(l),
                "right_terminal_progress_mean": terminal_progress(r),
                "left_wins": l["wins"],
                "right_wins": r["wins"],
                "scenario_count": l["sample_count"],
                "left_mean_clear_rate": l["mean_clear_rate"],
                "right_mean_clear_rate": r["mean_clear_rate"],
                "left_mean_final_stage": l["mean_final_stage"],
                "right_mean_final_stage": r["mean_final_stage"],
                "left_result_tower": tower_map(fixture, "plain" if lhs == "plain_values" else "baseline").get(key[0]),
                "right_result_tower": tower_map(fixture, "treasure" if rhs == "treasure_values" else "changed").get(key[0]),
            }
        )
    reversals = []
    for index, first in enumerate(common):
        for second in common[index + 1 :]:
            left_delta = terminal_progress(left[first]) - terminal_progress(left[second])
            right_delta = terminal_progress(right[first]) - terminal_progress(right[second])
            if left_delta * right_delta < 0:
                reversals.append(
                    {
                        "first": option_id(first),
                        "second": option_id(second),
                        "left_first_minus_second": left_delta,
                        "right_first_minus_second": right_delta,
                    }
                )
    return {
        "source_artifact": path.name,
        "scenario_seeds": data["scenario_seeds"],
        "candidate_count": len(common),
        "candidate_values": candidates,
        "rank_reversals_by_aggregate_mean": reversals,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--artifact-dir", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    treasure = summarize_pair(
        args.artifact_dir / "treasure-paired.json", "plain", "treasure", "black_white_owned"
    )
    deck = summarize_pair(
        args.artifact_dir / "deck-paired.json", "baseline", "changed", "draw_to_discard_card_id_46"
    )
    treasure["eight_scenario_confirmation"] = summarize_confirmation(
        args.artifact_dir / "treasure-confirmation.json",
        args.artifact_dir / "treasure-paired.json",
        "plain_values",
        "treasure_values",
    )
    deck["eight_scenario_confirmation"] = summarize_confirmation(
        args.artifact_dir / "deck-confirmation.json",
        args.artifact_dir / "deck-paired.json",
        "baseline_values",
        "changed_values",
    )
    actor_path = args.artifact_dir / "actor-response.json"
    actor_summary = None
    if actor_path.exists():
        actor = json.loads(actor_path.read_text())
        actor_summary = {
            "checkpoint": actor["actor_checkpoint"],
            "checkpoint_sha256": actor["actor_checkpoint_sha256"],
            "candidate_mode": actor["candidate_mode"],
            "input_contract": actor["input_contract"],
            "conditions": {},
        }
        for condition, left_key, right_key in (
            ("treasure", "plain_probabilities", "treasure_probabilities"),
            ("deck", "baseline_probabilities", "changed_probabilities"),
        ):
            values = actor[condition]
            left = {
                (tuple(row["card_ids"]), row["tower_slot"]): row["probability_within_build_tower_family"]
                for row in values[left_key]
            }
            right = {
                (tuple(row["card_ids"]), row["tower_slot"]): row["probability_within_build_tower_family"]
                for row in values[right_key]
            }
            shifts = [
                (abs(right[key] - left[key]), key, left[key], right[key])
                for key in sorted(set(left) & set(right))
            ]
            shifts.sort(reverse=True)
            actor_summary["conditions"][condition] = {
                "state_input_changed": values["actor_state_input_changed"],
                "effect_input_changed": values["actor_effect_input_changed"],
                "probability_shift_l1": sum(item[0] for item in shifts),
                "largest_single_option_shift": {
                    "candidate": option_id(shifts[0][1]),
                    "before": shifts[0][2],
                    "after": shifts[0][3],
                },
            }
    summary = {
        "schema_version": 1,
        "purpose": "raw per-scenario paired terminal progress summary; teacher victory bonus removed; 4 scenarios per condition",
        "paired_contexts": [treasure, deck],
        "actual_smoke_actor_forward": actor_summary,
        "learning": "not run: neither pair showed a stable value rank reversal; no labels or probabilities fabricated",
    }
    args.output.write_text(json.dumps(summary, indent=2) + "\n")


if __name__ == "__main__":
    main()
