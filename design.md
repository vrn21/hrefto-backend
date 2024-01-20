The System Design



API endpoints

GET /short_url
response{
    long_url
}

POST 301 or 304
body{
    long_url,
    short_text,
}

if short_text empty => create hash, should not coincide 
    we have solution: hash the link or use a probe hash table

success => get short_url
