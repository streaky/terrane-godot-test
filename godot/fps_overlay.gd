extends Label

const SAMPLE_INTERVAL := 0.5

@onready var simulation = get_node("../Simulation")
var elapsed := 0.0
var previous_steps := 0

func _ready() -> void:
	previous_steps = simulation.completed_steps()

func _process(delta: float) -> void:
	elapsed += delta
	if elapsed < SAMPLE_INTERVAL:
		return

	var current_steps: int = simulation.completed_steps()
	var simulation_fps := float(current_steps - previous_steps) / elapsed
	text = "Simulation: %.1f FPS\nVideo: %.1f FPS\nTime: %.0f×" % [
		simulation_fps,
		Engine.get_frames_per_second(),
		simulation.simulation_time_scale(),
	]
	previous_steps = current_steps
	elapsed = 0.0
