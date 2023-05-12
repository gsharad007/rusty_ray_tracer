Feature: Matrix Transformations

Scenario: Multiplying by a translation matrix
  Given transform ← translation(5, -3, 2)
    And p ← point(-3, 4, 5)
   Then transform * p = point(2, 1, 7)

Scenario: Multiplying by the inverse of a translation matrix
  Given transform ← translation(5, -3, 2)
    And inv ← inverse(transform)
    And p ← point(-3, 4, 5)
   Then inv * p = point(-8, 7, 3)

Scenario: Translation does not affect vectors
  Given transform ← translation(5, -3, 2)
    And v ← vector(-3, 4, 5)
   Then transform * v = v

Scenario: A scaling matrix applied to a point
  Given transform ← scaling(2, 3, 4)
    And p ← point(-4, 6, 8)
   Then transform * p = point(-8, 18, 32)

Scenario: A scaling matrix applied to a vector
  Given transform ← scaling(2, 3, 4)
    And v ← vector(-4, 6, 8)
   Then transform * v = vector(-8, 18, 32)

Scenario: Multiplying by the inverse of a scaling matrix
  Given transform ← scaling(2, 3, 4)
    And inv ← inverse(transform)
    And v ← vector(-4, 6, 8)
   Then inv * v = vector(-2, 2, 2)

Scenario: Reflection is scaling by a negative value
  Given transform ← scaling(-1, 1, 1)
    And p ← point(2, 3, 4)
   Then transform * p = point(-2, 3, 4)

