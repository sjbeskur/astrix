SELECT ppmxl_original_valid.ra
    ,ppmxl_original_valid.dec
    ,ppmxl_original_valid.b1mag
FROM gaiadr1.ppmxl_original_valid 
WHERE b1mag < 12