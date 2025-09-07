extends Node3D

func setExpr(expr: String):
	$ImplicitSurfaceGrapher.lhs_expr = expr
	$ImplicitSurfaceGrapher.rhs_expr = "4"
	$ImplicitSurfaceGrapher.eval()
