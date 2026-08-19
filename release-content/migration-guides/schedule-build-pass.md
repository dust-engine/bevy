---
title: Schedule Build Pass API changes
pull_requests: [19450]
---

The schedule build pass API was improved to provide more context and flexibility.

## Summary of Changes

- Breaking Change: The `ScheduleBuildPass` trait methods `collapse_set` and `build` have been
  updated with new parameters `(&mut World, &mut ScheduleGraph)` to provide more context.

- Breaking Change: `collapse_set` and `build` now have default (no-op) implementations, so passes
  only need to implement the hooks they care about.

- New Feature: A new method, `map_set_to_systems`, has been added to the `ScheduleBuildPass` trait,
  allowing passes to dynamically add systems to a set. Because sets are visited bottom-up, systems
  added here are inherited by the set's ancestors.

## Migration Steps

If you have custom implementations of `ScheduleBuildPass`, you will need to update the method
signatures for the `collapse_set` and `build` methods by adding the `world` and `graph` parameters.
