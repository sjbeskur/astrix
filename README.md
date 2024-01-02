# Astrix - Another STar RecognItion eXperiment

## Instructions:
Download the GAIA Star Catalog here:

[https://gea.esac.esa.int/archive/](https://gea.esac.esa.int/archive/)

Use the following query in the advanced ADQL query window and download the file as a csv:

``` sql
SELECT ppmxl_original_valid.ra
    ,ppmxl_original_valid.dec
    ,ppmxl_original_valid.b1mag
FROM gaiadr1.ppmxl_original_valid 
WHERE b1mag < 10
```

### Build, Run, Test

``` bash
# build (release mode) 
cargo build --release

# run 
cargo run <gaia_file.csv>
or
./target/release/astrix <gaia_file.csv>

# run tests and show output because my tests are amature hour
cargo test -- --nocapture
```

### Valgrind

``` bash
valgrind target/release/astrix  data/1680058493237O-result.csv 

==524901== HEAP SUMMARY:
==524901==     in use at exit: 0 bytes in 0 blocks
==524901==   total heap usage: 71,157 allocs, 71,157 frees, 71,090,482 bytes allocated
==524901== 
==524901== All heap blocks were freed -- no leaks are possible
==524901== 
==524901== For lists of detected and suppressed errors, rerun with: -s
==524901== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```


## Experimental Database support using Postgresql

project home directory run:

```
docker-compose -f docker/docker-compose.yml up
docker ps
docker inspect postgres
```

then run the migrations using:

```
sqlx migrate run
```


tear down
```
docker-compose -f docker/docker-compose.yml down --volumes
docker volumes ls 
docker volumes rm <name | --all>
```


### Notes

https://www.youtube.com/watch?v=TCERYbgvbq0&ab_channel=DreamsofCode


https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html

**Bulk Inserting:**

https://www.alxolr.com/articles/rust-bulk-insert-to-postgre-sql-using-sqlx