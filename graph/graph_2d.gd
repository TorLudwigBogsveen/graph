extends Node2D


func setExpr(expr: String):
	$LineGrapher.expr = expr
	$LineGrapher.clear_points()
	$LineGrapher.eval()
