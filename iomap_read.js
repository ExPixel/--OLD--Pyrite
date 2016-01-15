// MY EYES

function _map() {
	data8 = data8.map(function(item) {
  	return { "name": "self." + item[0], "offset": item[1] }
  });
  
	data16 = data16.map(function(item) {
  	return { "name": "self." + item[0], "offset": item[1] }
  });
  
	data32 = data32.map(function(item) {
  	return { "name": "self." + item[0], "offset": item[1] }
  });
  
	_map8();
  _map16();
  _map32();
  
  writeln("\nD8 Map\n===============\n");
  outputMap(d8map);
  writeln("\nD16 Map\n===============\n");
  outputMap(d16map);
  writeln("\nD32 Map\n===============\n");
  outputMap(d32map);
}

function outputMap(map) {
	for (key in map) {
  	var value = map[key];
    write("0x");
    write(parseInt(key).toString(16));
    write(" => ");
    write(value);
    writeln(",");
  }
}

function _map8() {
	data8.forEach(function(item) {
  	d8map[item.offset] = item.name;
  });
}

function _map16() {
	data16.forEach(function(item) {
  	d16map[item.offset] = item.name;
    d8map[item.offset] = shiftc(item.name, 0, 0xFF, 8);
    d8map[item.offset + 1] = shiftc(item.name, 8, 0xFF, 8);
  });
}

function _map32() {
	data32.forEach(function(item) {
  	d32map[item.offset] = item.name;
    d16map[item.offset] = shiftc(item.name, 0, 0xFFFF, 16);
    d16map[item.offset + 2] = shiftc(item.name, 16, 0xFFFF, 16);
    d8map[item.offset] = shiftc(item.name, 0, 0xFF, 8);
    d8map[item.offset + 1] = shiftc(item.name, 8, 0xFF, 8);
    d8map[item.offset + 2] = shiftc(item.name, 16, 0xFF, 8);
    d8map[item.offset + 3] = shiftc(item.name, 24, 0xFF, 8);
  });
}

function shiftc(a, shift, mask, type) {
  // if((typeof a) !== "number") { a = parseInt(a); }

  if(shift >  0) s = "(" + a + " >> " + shift + ")";
  else s = a;

  m = s + " & 0x" + mask.toString(16);
  t = "(" + m + ") as u" + type;
  return t;
}

var __obuf = "";
function write(t) {
  __obuf += t;
}

function writeln(t) {
  console.log(__obuf + t);
  __obuf = "";
}

var d8map = {};
var d16map = {};
var d32map = {};

var data8 = [
  ["POSTFLG", 0x4000300],
  ["HALTCNT", 0x4000301]
];

var data16 = [
  ["DISPCNT", 0x4000000],
  ["DISPSTAT", 0x4000004],
  ["VCOUNT", 0x4000006],
  ["BG0CNT", 0x4000008],
  ["BG1CNT", 0x400000a],
  ["BG2CNT", 0x400000c],
  ["BG3CNT", 0x400000e],
  ["BG0HOFS", 0x4000010],
  ["BG0VOFS", 0x4000012],
  ["BG1HOFS", 0x4000014],
  ["BG1VOFS", 0x4000016],
  ["BG2HOFS", 0x4000018],
  ["BG2VOFS", 0x400001a],
  ["BG3HOFS", 0x400001c],
  ["BG3VOFS", 0x400001e],
  ["BG2PA", 0x4000020],
  ["BG2PB", 0x4000022],
  ["BG2PC", 0x4000024],
  ["BG2PD", 0x4000026],
  ["BG3PA", 0x4000030],
  ["BG3PB", 0x4000032],
  ["BG3PC", 0x4000034],
  ["BG3PD", 0x4000036],
  ["WIN0H", 0x4000040],
  ["WIN1H", 0x4000042],
  ["WIN0V", 0x4000044],
  ["WIN1V", 0x4000046],
  ["WININ", 0x4000048],
  ["WINOUT", 0x400004a],
  ["MOSAIC", 0x400004c],
  ["BLDCNT", 0x4000050],
  ["BLDALPHA", 0x4000052],
  ["BLDY", 0x4000054],
  ["SOUND1CNT_L", 0x4000060],
  ["SOUND1CNT_H", 0x4000062],
  ["SOUND1CNT_X", 0x4000064],
  ["SOUND2CNT_L", 0x4000068],
  ["SOUND2CNT_H", 0x400006c],
  ["SOUND3CNT_L", 0x4000070],
  ["SOUND3CNT_H", 0x4000072],
  ["SOUND3CNT_X", 0x4000074],
  ["SOUND4CNT_L", 0x4000078],
  ["SOUND4CNT_H", 0x400007c],
  ["SOUNDCNT_L", 0x4000080],
  ["SOUNDCNT_H", 0x4000082],
  ["SOUNDCNT_X", 0x4000084],
  ["SOUNDBIAS", 0x4000088],
  ["WAVE_RAM0_L", 0x4000090],
  ["WAVE_RAM0_H", 0x4000092],
  ["WAVE_RAM1_L", 0x4000094],
  ["WAVE_RAM1_H", 0x4000096],
  ["WAVE_RAM2_L", 0x4000098],
  ["WAVE_RAM2_H", 0x400009a],
  ["WAVE_RAM3_L", 0x400009c],
  ["WAVE_RAM3_H", 0x400009e],
  ["FIF0_A_L", 0x40000a0],
  ["FIFO_A_H", 0x40000a2],
  ["FIFO_B_L", 0x40000a4],
  ["FIFO_B_H", 0x40000a6],
  ["DMA0CNT_L", 0x40000b8],
  ["DMA0CNT_H", 0x40000ba],
  ["DMA1CNT_L", 0x40000c4],
  ["DMA1CNT_H", 0x40000c6],
  ["DMA2CNT_L", 0x40000d0],
  ["DMA2CNT_H", 0x40000d2],
  ["DMA3CNT_L", 0x40000dc],
  ["DMA3CNT_H", 0x40000de],
  ["TM0CNT_L", 0x4000100],
  ["TM0CNT_H", 0x4000102],
  ["TM1CNT_L", 0x4000104],
  ["TM1CNT_H", 0x4000106],
  ["TM2CNT_L", 0x4000108],
  ["TM2CNT_H", 0x400010a],
  ["TM3CNT_L", 0x400010c],
  ["TM3CNT_H", 0x400010e],
  ["SIOMULTI0", 0x4000120],
  ["SIOMULTI1", 0x4000122],
  ["SIOMULTI2", 0x4000124],
  ["SIOMULTI3", 0x4000126],
  ["SIOCNT", 0x4000128],
  ["SIOMLT_SEND", 0x400012a],
  ["KEYINPUT", 0x4000130],
  ["KEYCNT", 0x4000132],
  ["RCNT", 0x4000134],
  ["IR", 0x4000136],
  ["JOYCNT", 0x4000140],
  ["JOY_STAT", 0x4000158],
  ["IE", 0x4000200],
  ["IF", 0x4000202],
  ["WAITCNT", 0x4000204],
  ["IME", 0x4000208]
];

var data32 = [
  ["BG2X", 0x4000028],
  ["BG2Y", 0x400002c],
  ["BG3X", 0x4000038],
  ["BG3Y", 0x400003c],
  ["FIFO_A", 0x40000a0],
  ["FIFO_B", 0x40000a4],
  ["DMA0SAD", 0x40000b0],
  ["DMA0DAD", 0x40000b4],
  ["DMA1SAD", 0x40000bc],
  ["DMA1DAD", 0x40000c0],
  ["DMA2SAD", 0x40000c8],
  ["DMA2DAD", 0x40000cc],
  ["DMA3SAD", 0x40000d4],
  ["DMA3DAD", 0x40000d8],
  ["SIODATA32", 0x4000120],
  ["JOY_RECV", 0x4000150],
  ["JOY_TRANS", 0x4000154]
];

_map();
if(__obuf && __obuf.length > 0) {
  console.log(__obuf);
  __obuf = "";
}