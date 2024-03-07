// pub mod db{
//     use postgres::Client;
//     use postgres::Error;
//     use postgres::NoTls;
//     struct Link {
//         long_url: &str,
//         short_url: &str,
//     }

//     pub fn select(client: &mut Client, s_url: &str) -> Link {
//         let db_query = client.query("SELECT longurl FROM hrefto WHERE shorturl = $1", &[&s_url]).unwrap();

//         let link = Link {
//             long_url: db_query.get(0).get(0),
//             short_url: s_url,
//         };

//         link
//     }

//  pub fn insert(client:&mut Client, l_url: &str, s_url: &str) -> Link{
//     let link;
//     if check(client, l_url, s_url) == 0{
//         link = Link{
//             long_url: l_url,
//             short_url: s_url
//         };

//         client.execute("INSERT INTO hrefto (longurl,shorturl) VALUES ($1, $2)", &link.long_url,&link.short_url);
//     }else{
//         link = select(client, s_url);
//     }
//     link
//  }

// pub fn check(client:&mut Client, l_url: &str, s_url: &str) -> i8{
//     let db_link = select(client, s_url);
//     if db_link.long_url == l_url {
//         1
//     } else {
//         0
//     }
// }


// }
pub mod db {
    use postgres::Client;
    use postgres::Error;
    use postgres::NoTls;

    pub struct Link<'a> {
        long_url: &'a str,
        short_url: &'a str,
    }

    pub fn select<'a>(client: &'a mut Client, s_url: &'a str) -> Link<'a> {
        let db_query = client
            .query("SELECT longurl FROM hrefto WHERE shorturl = $1", &[&s_url])
            .unwrap();

        let link = Link {
            long_url: db_query.get(0).unwrap().get::<usize, &str>(0),
            short_url: s_url,
        };

        link
    }

    pub fn insert<'a>(client: &'a mut Client, l_url: &'a str, s_url: &'a str) -> Link<'a> {
        let link;
        if check(client, l_url, s_url) == 0 {
            link = Link {
                long_url: l_url,
                short_url: s_url,
            };

            client.execute(
                "INSERT INTO hrefto (longurl,shorturl) VALUES ($1, $2)",
                &[&link.long_url, &link.short_url],
            );
        } else {
            link = select(client, s_url);
        }
        link
    }

    pub fn check(client: &mut Client, l_url: &str, s_url: &str) -> i8 {
        let db_link = select(client, s_url);
        if db_link.long_url == l_url {
            1
        } else {
            0
        }
    }
}
