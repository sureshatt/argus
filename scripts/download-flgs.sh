#!/bin/bash

# Output directory for flags
OUTPUT_DIR="../public/flags/"
mkdir -p "$OUTPUT_DIR"

# List of all ISO 3166-1 alpha-2 country codes (lowercase)
country_codes=(
  ad ae af ag ai al am ao aq ar as at au aw ax az
  ba bb bd be bf bg bh bi bj bl bm bn bo bq br bs bt bv bw by bz
  ca cc cd cf cg ch ci ck cl cm cn co cr cu cv cw cx cy cz
  de dj dk dm do dz
  ec ee eg eh er es et
  fi fj fm fo fr
  ga gb gd ge gf gg gh gi gl gm gn gp gq gr gt gu gw gy
  hk hm hn hr ht hu
  id ie il im in io iq ir is it
  je jm jo jp
  ke kg kh ki km kn kp kr kw ky kz
  la lb lc li lk lr ls lt lu lv ly
  ma mc md me mf mg mh mk ml mm mn mo mp mq mr ms mt mu mv mw mx my mz
  na nc ne nf ng ni nl no np nr nu nz
  om
  pa pe pf pg ph pk pl pm pn pr pt pw py
  qa
  re ro rs ru rw
  sa sb sc sd se sg sh si sj sk sl sm sn so sr ss st sv sx sy sz
  tc td tf tg th tj tk tl tm tn to tr tt tv tz
  ua ug um us uy uz
  va vc ve vg vi vn vu
  wf ws
  ye yt
  za zm zw
)

# Base URL
BASE_URL="https://flagcdn.com"

# Download each flag
for code in "${country_codes[@]}"; do
  URL="${BASE_URL}/${code}.svg"
  OUTPUT_FILE="${OUTPUT_DIR}/${code}.svg"
  
  echo "Downloading $code..."
  
  curl -s --fail "$URL" -o "$OUTPUT_FILE"
  
  if [ $? -ne 0 ]; then
    echo "Failed to download $code"
  fi
done

echo "✅ All downloads attempted. Flags saved in ./${OUTPUT_DIR}/"
