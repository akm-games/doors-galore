extends KinematicBody2D

export (int) var speed = 300

var velocity = Vector2();
var is_moving_right = false

func move_right():
  velocity = Vector2();
  velocity.x = speed

func get_input():
  velocity = Vector2()
  if Input.is_action_pressed("right"):
    velocity.x += 1
  if Input.is_action_pressed("left"):
    velocity.x -= 1
  if Input.is_action_pressed("down"):
    velocity.y += 1
  if Input.is_action_pressed("up"):
    velocity.y -= 1
  velocity = velocity.normalized() * speed

func _init():
  is_moving_right = false

func _ready():
  var vec2_viewport = get_viewport_rect().size
  position = vec2_viewport / 2;


func _physics_process(delta):
  velocity = Vector2()
  if is_moving_right:
    move_right()
  velocity = move_and_collide(velocity * delta)

func _on_Door_1_input_event(viewport, event, shape_idx):
  if event.is_action_released("select"):
    is_moving_right = true


func _on_Door_2_input_event(viewport, event, shape_idx):
  if event.is_action_released("select"):
    is_moving_right = true


func _on_Door_3_input_event(viewport, event, shape_idx):
  if event.is_action_released("select"):
    is_moving_right = true
