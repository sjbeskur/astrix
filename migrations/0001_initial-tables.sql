-- Add migration script here
create table GaiaCatalog (
  id bigserial primary key,
  ra double precision not null,
  dec double precision not null,
  x double precision not null,
  y double precision not null,
  z double precision not null
  --vin varchar(255) not null,
  --constraint fk_location foreign key (location_id) references locations(id) on delete cascade
);