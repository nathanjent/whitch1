<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.10" tiledversion="1.10.2" name="bg" tilewidth="8" tileheight="8" tilecount="1024" columns="32">
 <transformations hflip="0" vflip="0" rotate="0" preferuntransformed="1"/>
 <image source="../gfx/bg.png" trans="ff00ff" width="256" height="256"/>
 <tile id="34" probability="1.5"/>
 <tile id="37" probability="1.5"/>
 <tile id="38" probability="1.5"/>
 <tile id="39" probability="0.5"/>
 <wangsets>
  <wangset name="ground-grassy" type="mixed" tile="-1">
   <wangcolor name="grass" color="#00ff00" tile="-1" probability="1"/>
   <wangtile tileid="4" wangid="0,0,1,1,1,0,0,0"/>
   <wangtile tileid="5" wangid="0,0,1,1,1,1,1,0"/>
   <wangtile tileid="6" wangid="0,0,1,1,1,1,1,0"/>
   <wangtile tileid="7" wangid="0,0,1,1,1,1,1,0"/>
   <wangtile tileid="8" wangid="0,0,0,0,1,1,1,0"/>
   <wangtile tileid="36" wangid="1,1,1,1,1,0,0,0"/>
   <wangtile tileid="37" wangid="1,1,1,1,1,1,1,1"/>
   <wangtile tileid="38" wangid="1,1,1,1,1,1,1,1"/>
   <wangtile tileid="39" wangid="1,1,1,0,0,0,1,1"/>
   <wangtile tileid="40" wangid="1,0,0,0,1,1,1,1"/>
   <wangtile tileid="68" wangid="1,1,1,1,1,1,1,0"/>
   <wangtile tileid="69" wangid="1,1,1,0,1,1,1,1"/>
   <wangtile tileid="70" wangid="1,1,1,1,1,0,1,1"/>
   <wangtile tileid="71" wangid="1,1,1,1,1,1,1,1"/>
   <wangtile tileid="72" wangid="1,0,1,1,1,1,1,1"/>
   <wangtile tileid="101" wangid="1,0,0,0,0,0,1,1"/>
   <wangtile tileid="102" wangid="1,1,1,0,0,0,0,0"/>
  </wangset>
  <wangset name="wall" type="edge" tile="-1">
   <wangcolor name="brick" color="#ff0000" tile="-1" probability="1"/>
   <wangtile tileid="1" wangid="0,0,1,0,1,0,0,0"/>
   <wangtile tileid="2" wangid="0,0,1,0,1,0,1,0"/>
   <wangtile tileid="3" wangid="0,0,0,0,1,0,1,0"/>
   <wangtile tileid="33" wangid="1,0,1,0,1,0,0,0"/>
   <wangtile tileid="34" wangid="1,0,1,0,1,0,1,0"/>
   <wangtile tileid="35" wangid="1,0,0,0,1,0,1,0"/>
   <wangtile tileid="65" wangid="1,0,1,0,0,0,0,0"/>
   <wangtile tileid="66" wangid="1,0,1,0,0,0,1,0"/>
   <wangtile tileid="67" wangid="1,0,0,0,0,0,1,0"/>
  </wangset>
 </wangsets>
</tileset>
