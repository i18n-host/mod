#!/usr/bin/env coffee

> path > join dirname
  fs > existsSync rmSync
  @3-/read
  @3-/write
  # @3-/nt/load.js
  @3-/mysql2rust/sqlLi.js
  @3-/mysql2rust/rm.js > rm rmPre
  @3-/mysql2rust/gener.js
  @3-/mysql2rust/rust.js

firstUpperCase = (str) =>
  for ch, i in str
    if ch == ch.toUpperCase()
      return i
  return -1


await do =>
  [DUMP_SQL] = process.argv.slice(2)

  if not DUMP_SQL
    console.error 'miss arg xxx.sql'
    return

  root = dirname(dirname(dirname(import.meta.dirname)))
  [GEN, gen] = gener()
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

    li = mod.get(prefix)
    if not li
      li = []
      mod.set(prefix, li)
    li.push [
      kind
      dump_name
      sql
    ]

  for [mod_name, li] from mod.entries()
    GEN.splice(0,GEN.length)
    console.log '# '+mod_name
    for [kind,dump_name,sql] from li
      gen(
        kind
        dump_name
        sql
      )
    for i in [mod_name, mod_name+'_']
      dir = join(root,i)
      if existsSync(dir)
        write(
          join dir,'src/db.rs'
          rust GEN
        )
        break

  console.log root
  return
  #   gen(
  #     kind
  #     dump_name
  #     sql
  #   )
  #
  # console.log rust GEN
  return

# nt = load MOD+'.nt'
#
#
# if r.length
#   mod = new Map
#   for i from nt
#     p = i.lastIndexOf '/'
#     if p
#       k = i.slice(p+1)
#     else
#       k = i
#     mdir = join i,'db'
#     mod.set k, mdir
#     rmPre join MOD,mdir
#
#   DUMP_DIR = join ROOT, 'db'
#
#   rmPre DUMP_DIR
#
#   for [kind,name,sql] from r
#     p = firstUpperCase(name)
#     if ~p
#       prefix = name.slice(0,p)
#       dump_name = name.slice(p)
#     else
#       prefix = dump_name = name
#
#     m = mod.get(prefix)
#     if m
#       gen kind,name,sql
#       write(
#         join MOD, m, kind, dump_name+'.sql'
#         sql
#       )
#       continue
#     gen kind,name,sql
#     write(
#       join(DUMP_DIR, kind, name+'.sql')
#       sql
#     )
#
# write(
#   join ROOT, 'rust/lib/m/src/lib.rs'
#   rust GEN
# )
