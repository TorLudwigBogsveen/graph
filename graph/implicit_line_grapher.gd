extends ImplicitLineGrapher

var canvas_rid

func _ready():
	canvas_rid = RenderingServer.canvas_item_create()
	RenderingServer.canvas_item_set_parent(canvas_rid, get_canvas_item())

func draw_lines(points: PackedVector2Array, color: Color, width: float):
	RenderingServer.free_rid(canvas_rid)
	canvas_rid = RenderingServer.canvas_item_create()
	RenderingServer.canvas_item_set_parent(canvas_rid, get_canvas_item())
	# lines = array of [Vector2 start, Vector2 end] pairs
	var i = 0
	while i < points.size():
		RenderingServer.canvas_item_add_line(canvas_rid, points[i], points[i+1], color, width)
		i += 2
