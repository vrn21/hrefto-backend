pub mod db{
    use postgresql::{Client,Error,NoTls};
    use postgresql::types::Type;

 struct Link{
    long_url: &str,
    short_url:&str
 }

 pub fn SELECT(client:&mut Client, s_url: &str) -> link{
    let db_query = client.query("SELECT longurl FROM hrefto WHERE shorturl = $1",s_url);
    
    let link = Link {
        long_url: db_query.get(),
        short_url: s_url
    };

    link
 }

 pub fn INSERT(client:&mut Client, l_url: &str, s_url: &str) -> link{

    if CHECK(client, l_url, s_url) == 0{
        let link = Link{
            long_url: l_url,
            short_url: s_url
        };

        client.execute("INSERT INTO hrefto (longurl,shorturl) VALUES ($1, $2)", &link.long_url,&link.short_url);
    }else{
        let link = SELECT(client, s_url);
    }
    link
 }

 pub fn CHECK(client:&mut Client, l_url: &str, s_url: &str) -> i8{
    let db_link = SELECT(client, s_url);
    if db_link == l_url {
        1
    } 0
}


}