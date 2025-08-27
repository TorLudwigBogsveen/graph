extends Node2D


func setExpr(expr: String):
	$LineGrapher.expr = expr
	$LineGrapher.clear_points()
	$LineGrapher.eval()
	$ImplicitLineGrapher.lhs_expr = "mul(y,x)"
	$ImplicitLineGrapher.rhs_expr = expr
	$ImplicitLineGrapher.eval()
	$ImplicitLineGrapher.draw_lines($ImplicitLineGrapher.lines, Color.RED, 2)
