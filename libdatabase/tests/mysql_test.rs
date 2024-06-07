use libdatabase::mysql_sql_ttl;

/// 全生命周期 SQL 语句生成测试
#[tokio::test]
async fn test_mysql_sql_parser() -> anyhow::Result<()> {

    dbg!(mysql_sql_ttl("INSERT INTO employees (id, name, salary) VALUES ('10000', 'Alice', 70000),('10001', 'Green', 70001)".to_string()).await?);
    dbg!(mysql_sql_ttl("UPDATE employees SET salary = 70007 WHERE id = '10001'".to_string()).await?);

    Ok(())
}

