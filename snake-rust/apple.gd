extends Apple



func _on_area_entered(area: Area2D) -> void:
	if area is Snake:
		self.queue_free()
