const float v = 1.0;

const vec2 verts[4] = vec2[4](
	vec2( v,  v),
	vec2(-v,  v),
	vec2( v, -v),
	vec2(-v, -v)
);
layout (std140) uniform Camera {
	mat4 projection;
	mat4 view;
};
uniform mat4 transform;
void main() {
	vec2 vert = verts[gl_VertexID];
	gl_Position = projection * view * transform * vec4(vert, 0.0, 1.0);
}
