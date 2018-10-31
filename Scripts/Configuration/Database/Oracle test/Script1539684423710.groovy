

CustomKeywords.'database.DataBaseUtils.connectDB'('192.168.8.52', 'orcl', '1521', 'qp_central', 'qp_central')

def oid = CustomKeywords.'database.DataBaseUtils.executeMatchQuery'('select id from qp_central.branch where name = \'Mölndal\'', 
    'id')

println(oid)

