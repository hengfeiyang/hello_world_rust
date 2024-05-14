 
mod db;
use db::Db; 

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let db = db::get_db().await;
    db.create_table().await?;
    let ret = db.get("abc").await?;
    println!("get ret: {:?}", ret);
    let out_var = "abc".to_string();
    db.put_with_callback("/123", db::NEED_WATCH, None, Box::new(move |v|{
        println!("callback: {:?}", v);
        Ok(Some(bytes::Bytes::from(out_var)))
    })).await?;

    let mut m = indexmap::IndexMap::new();
    m.insert("a", "1");
    m.insert("e", "1");
    m.insert("b", "1");
    m.insert("e", "2");
    m.insert("c", "1");
    m.insert("a", "2");
    let keys = m.keys().map(|k|k.to_owned()).collect::<Vec<_>>();
    assert_eq!(keys,["a", "e", "b", "c"]);
    let values = m.values().map(|k|k.to_owned()).collect::<Vec<_>>();
    assert_eq!(values, ["2", "2", "1", "1"]);
    Ok(())
}
