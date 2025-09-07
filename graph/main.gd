extends Node2D


func _on_text_edit_text_changed() -> void:
	#$Graph2D.setExpr($TextEdit.text)
	$Graph3D.setExpr($TextEdit.text)
	pass # Replace with function body.
