const float v = 0.25;

const vec2 verts[4] = vec2[4](
	vec2( v,  v),
	vec2(-v,  v),
	vec2( v, -v),
	vec2(-v, -v)
);
uniform mat4 projection;
uniform mat4 view;
uniform mat4 transform;
void main() {
	vec2 vert = verts[gl_VertexID];
	gl_Position = projection * view * transform * vec4(vert, 0.0, 1.0);
}
