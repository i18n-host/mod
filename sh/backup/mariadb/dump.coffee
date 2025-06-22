#!/usr/bin/env coffee

> path > join dirname
  fs > existsSync rmSync
  @3-/read
  @3-/rm
  @3-/write
  @3-/mysql2rust/sqlLi.js
  @3-/mysql2rust/gener.js
  @3-/mysql2rust/rust.js
  @3-/default:

firstUpperCase = (str) =>
  for ch, i in str
    if ch == ch.toUpperCase()
      return i
  return -1

[GEN, gen] = gener()
PWD = import.meta.dirname
ROOT = dirname(dirname(dirname(PWD)))

dumpSql = (dir, li) =>
  dir_db = join dir,'db'
  rm(dir_db)
  for [kind,dump_name,sql] from li
    write(
      join dir_db, kind, dump_name+'.sql'
      sql
    )
  return

genRs = (mod_name, li) =>
  for i in [mod_name, mod_name+'_']
    GEN.splice(0,GEN.length)
    dir = join(ROOT,i)
    if existsSync(dir)
      dumpSql dir, li
      for [kind,dump_name,sql] from li
        gen(
          kind
          dump_name
          sql
        )
      write(
        join dir,'src/db.rs'
        rust mod_name, GEN
      )
      return 1
  return

await do =>
  [DUMP_SQL] = process.argv.slice(2)

  if not DUMP_SQL
    console.error 'miss arg xxx.sql'
    return

  r = sqlLi read(DUMP_SQL)

  mod = new Map

  for [kind,name,sql] from r
    p = firstUpperCase(name)
    if ~p
      prefix = name.slice(0,p)
      dump_name = name.slice(p)
    else
      prefix = dump_name = name

    dump_name = dump_name.charAt(0).toLowerCase() + dump_name.slice(1)

    mod.default(prefix, =>[]).push [
      kind
      dump_name
      sql
    ]

  for [mod_name, li] from mod.entries()
    console.log '# '+mod_name
    if not genRs(mod_name, li)
      dumpSql ROOT, li

  return


