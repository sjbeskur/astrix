select ra, dec
	, cos(dec) * cos(ra) as xx
	, cos(dec) * sin(ra) as yy
	, sin(dec) as zz
from gaiacatalog
where mag < 4 LIMIT 1000;

select min(mag) from gaiacatalog
select max(mag) from gaiacatalog
where mag < 3 and mag > 1 LIMIT 1000;


truncate table gaiacatalog;

SELECT pg_size_pretty(pg_total_relation_size('"public"."gaiacatalog"'));