#version 140

// The Gamma of the GBA screen.
// Got these values from:
// http://codewitchgamedev.blogspot.com/2015/08/emulating-gbas-display-with-gamma.html
#define GAMMA_R 4.0
#define GAMMA_G 3.0
#define GAMMA_B 1.4

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() {
	vec4 tcolor = texture(tex, v_tex_coords);
	tcolor.rgb = vec3(pow(tcolor.r, GAMMA_R), pow(tcolor.g, GAMMA_G), pow(tcolor.b, GAMMA_B));
    color = tcolor;
}