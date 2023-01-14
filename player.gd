extends Sprite

func center_sprite():
	var vec2_viewport = get_viewport_rect().size
	position = Vector2(vec2_viewport.x/2, vec2_viewport.y/2)

func _ready():
	center_sprite()
	get_tree().get_root().connect("size_changed", self, "center_sprite")
