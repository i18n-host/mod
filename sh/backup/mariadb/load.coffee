#!/usr/bin/env coffee

> path > join resolve basename dirname
  fs > existsSync readdirSync statSync
  zx/globals:
  @3-/dbq > $:$db $one
  @3-/read
  @3-/write
  os > tmpdir
  which
  @iarna/toml > parse

PWD = import.meta.dirname
ROOT = resolve PWD, '../../..'
TABLE = 'table'
$.verbose = 1

{
  MYSQL_DB
  MYSQL_HOST
  MYSQL_PWD
  MYSQL_PORT
  MYSQL_USER
} = process.env

mariadb = 'mariadb'
mariadb = if await which(mariadb, { nothrow: true }) then mariadb else 'mysql'

importSql = (sql)=>
  $"#{mariadb} -h #{MYSQL_HOST} -P#{MYSQL_PORT} -u #{MYSQL_USER} #{MYSQL_DB} < #{sql}"

loadSql = (dir)=>
  console.log dir
  li = []
  for i from readdirSync dir
    if i.endsWith '.sql'
      fp = join dir, i
      li.push '-- '+fp
      sql = read(fp).replaceAll(
        'CREATE TABLE ',
        'CREATE TABLE IF NOT EXISTS'
      )
      if sql.startsWith 'CREATE FUNCTION '
        t = sql.slice(16)
        name = t.slice(0,t.indexOf('('))
        sql = 'DROP FUNCTION IF EXISTS '+name+';\n'+sql

      li.push sql

  tmpfp = join tmpdir(), 'import.sql'
  write(tmpfp, 'START TRANSACTION;'+li.join('\n')+'COMMIT;')
  # await importSql tmpfp
  return

load = (dir)=>
  dir_db = join dir, 'db'
  if not existsSync dir_db
    return
  li = new Set readdirSync dir_db
  if li.delete(TABLE)
    table_dir = join dir_db, TABLE
    await loadSql(table_dir)
  for i from li
    await loadSql(join dir_db, i)
  return

await Promise.all parse(read join(ROOT,'Cargo.toml')).workspace.members.map (i)=>
  load(join(ROOT, i))

await load PWD

# scan = (dir)=>
#   if not existsSync dir
#     return
#   dirli = readdirSync dir
#   table = 'table'
#   p = dirli.indexOf(table)
#   if p > 0
#     dirli.splice(p, 1)
#     dirli.unshift table
#   for subdir from dirli
#     ndir = join(dir,subdir)
#     if statSync(ndir).isFile()
#       continue
#
#     for await i from walk ndir
#       if i.endsWith '.sql'
#         rfp = i.slice(ROOT.length+1)
#         console.log greenBright rfp
#         sql = read i
#         kind = basename dirname(rfp)
#         len = ('CREATE '+kind).length + 1
#
#         if ['function','procedure','trigger'].includes(kind)
#           if kind == 'trigger'
#             end = ' '
#           else
#             end = '('
#           name = sql.slice(len,sql.indexOf(end,len))
#
#           li = [
#             sql
#           ]
#
#           # hack for https://github.com/oceanbase/oceanbase/issues/1817 oceanbase 不支持 DROP procedure IF EXISTS cronLi
#           if kind == 'procedure'
#             name = name.replaceAll('`','')
#             if await $one("SELECT COUNT(1) FROM information_schema.routines WHERE routine_name='#{name}' AND ROUTINE_TYPE='PROCEDURE' AND ROUTINE_SCHEMA=?",process.env.MYSQL_DB)
#               li.unshift(
#                 "DROP PROCEDURE #{name};"
#               )
#           else
#             li.unshift(
#               "DROP #{kind} IF EXISTS #{name};"
#             )
#         else
#           li = sql.split(';\n').filter((i)=>i.length).map(
#             (i)=>
#               CREATE_TABLE ='CREATE TABLE '
#               if i.startsWith(CREATE_TABLE)
#                 i = CREATE_TABLE+" IF NOT EXISTS " + i.slice(13)
#               i+';'
#           )
#         await $db(
#           (c)=>
#             # hack for https://github.com/oceanbase/oceanbase/issues/1818
#             if kind == 'procedure'
#               {tmpdir} = await import('os')
#               {rmSync} = await import('fs')
#
#               write = (await import('@3-/write')).default
#               tdir = join(tmpdir(),'sql')
#               fp = join(tdir,name+'.sql')
#               write(
#                 fp
#                 'delimiter ;;\n'+li.join(';\n')+'\ndelimiter ;'
#               )
#               await importSql(fp)
#               rmSync(fp)
#               return
#
#             await c.beginTransaction()
#             if kind == 'table'
#               c.query(
#                 'SET SESSION default_storage_engine=\'RocksDB\''
#               )
#             for i from li
#               await c.query(i)
#             await c.commit()
#             return
#         )
#
#   init_sql = join dir,'init.sql'
#
#   if existsSync init_sql
#     importSql init_sql
#   return
#
# ing = [
#   scan join ROOT,'db'
# ]
# for i from load join ROOT, 'mod.nt'
#   ing.push scan join ROOT,'mod',i,'db'
#
# await Promise.all ing
#
process.exit()
