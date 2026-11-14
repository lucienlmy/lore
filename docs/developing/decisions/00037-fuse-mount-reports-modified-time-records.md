---
status: accepted
date: 2026-11-14
deciders: Mattias Jansson
---

# ADR-00037: A virtual clone's mount reports its modified-time records

## Context and Problem Statement

[ADR-00029](00029-instance-local-modified-time-records.md) keeps modified-time records in the instance's
dot directory, as a cache that lets a scan skip hashing a file: a lost record costs a hash, never a wrong
answer.

A FUSE virtual clone serves its revision's files from the store, so a sync or switch writes no file
and no file takes a new modification time from the filesystem. Its mount reported, for each path, the time
the *swap* that last changed the path recorded, and v0.14.0 stored those times in the mutable store under
keys named by the instance (`fuse-recorded-time`, with a `fuse-recorded-times` marker), which every new
inode's lookup read. A copy of the instance, as the
[copy-on-write instance clone](../../proposals/2026-10-31-copy-on-write-instance-clone.md) makes, takes a
new instance ID and cannot carry them. The same swap also recorded the same times in the records, and a FUSE
instance tells a changed file by address, so its records answered nothing its scans needed.

## Decision Drivers

- A copy of a virtual clone reports the times its source reports, so a build tool rebuilds what changed.
- A swap writes each time once, and a lookup reads no store.
- Whether a file changed does not depend on its time.

## Considered Options

- Keep the swap times in the mutable store, keyed by the instance, and copy them for the new instance
- Keep the swap times in a directory of their own beside the records, in their format
- Report the records' times, and compare a virtual clone's files by address and content only

## Decision Outcome

Chosen option: "Report the records' times, and compare a virtual clone's files by address and content
only", because the records are already per instance, keyed by the path, and carried by a copy, and in a
virtual clone they hold nothing else.

Records live in `.lore/records/` as ADR-00029 lays them out: keyed by the path, 256 shards of 24-byte
records, a new instance marked as holding no store records, an older instance falling back to the store
records an older client wrote, which an upgraded client zeroes as it records or clears a path, and SWFS
and in-memory instances keeping theirs in the store. In addition:

- A virtual clone's mount reports, for a file it serves from the store, the time its records hold for the
  path, and the instance's base time where they hold none. A swap records its time for each path it
  changes and writes the records back before it publishes, synced under `--sync-data`. A commit records
  the modification time it read for each file it stores, however recent.
- A provider that holds its working tree as a state tree, as the FUSE provider does, compares a file by
  address and content, never by a recorded time, and records no time from a comparison.
- A process opens one set of records per dot directory, found by its canonical path, which the commands
  holding the instance's repository lock and the instance's mount share.

The [instance-local modified-time records](../../proposals/2026-11-03-instance-local-modified-time-records.md)
LEP gives the design.

### Consequences

- Good, because cloning a virtual clone carries the times its mount reports.
- Good, because a swap writes its times once, and a lookup reads a shard in memory, never the store.
- Good, because a committed file keeps the time it showed across a remount.
- Good, because no two files with the same size and time hide a change in a virtual clone.
- Bad, because in a virtual clone a lost record is no longer only a hash: the mount reports the base time,
  older than the content, and a build tool can skip a rebuild. A commit's times are written back with the
  command's flush, unsynced, after the commit serves the files from the store.
- Bad, because v0.14.0, after a downgrade, reads only its store keys and reports older times for the paths
  an upgraded swap changed, so a virtual clone needs a clean build after a downgrade.
- Bad, because a mount keeps its records in memory for as long as it serves, so records a failed command
  left changed stay there until a later write-back.

## More Information

Supersedes [ADR-00029](00029-instance-local-modified-time-records.md), whose decision this one carries
forward for instances on disk.
