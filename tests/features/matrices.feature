Feature: Matrices

  Scenario: Constructing and inspecting a 4x4 matrix
    Given the following 4x4 matrix M:
      |    1 |    2 |    3 |    4 |
      |  5.5 |  6.5 |  7.5 |  8.5 |
      |    9 |   10 |   11 |   12 |
      | 13.5 | 14.5 | 15.5 | 16.5 |
    Then M[0,0] = 1
    And M[0,3] = 4
    And M[1,0] = 5.5
    And M[1,2] = 7.5
    And M[2,2] = 11
    And M[3,0] = 13.5
    And M[3,2] = 15.5

  Scenario: A 2x2 matrix ought to be representable
    Given the following 2x2 matrix M:
      | -3 |  5 |
      |  1 | -2 |
    Then M[0,0] = -3
    And M[0,1] = 5
    And M[1,0] = 1
    And M[1,1] = -2

  Scenario: A 3x3 matrix ought to be representable
    Given the following 3x3 matrix M:
      | -3 |  5 |  0 |
      |  1 | -2 | -7 |
      |  0 |  1 |  1 |
    Then M[0,0] = -3
    And M[1,1] = -2
    And M[2,2] = 1

  Scenario: Matrix equality with identical matrices
    Given the following matrix A:
      | 1 | 2 | 3 | 4 |
      | 5 | 6 | 7 | 8 |
      | 9 | 8 | 7 | 6 |
      | 5 | 4 | 3 | 2 |
    And the following matrix B:
      | 1 | 2 | 3 | 4 |
      | 5 | 6 | 7 | 8 |
      | 9 | 8 | 7 | 6 |
      | 5 | 4 | 3 | 2 |
    Then A = B

  Scenario: Matrix equality with different matrices
    Given the following matrix A:
      | 1 | 2 | 3 | 4 |
      | 5 | 6 | 7 | 8 |
      | 9 | 8 | 7 | 6 |
      | 5 | 4 | 3 | 2 |
    And the following matrix B:
      | 2 | 3 | 4 | 5 |
      | 6 | 7 | 8 | 9 |
      | 8 | 7 | 6 | 5 |
      | 4 | 3 | 2 | 1 |
    Then A != B

  Scenario: Multiplying two matrices
    Given the following matrix A:
      | 1 | 2 | 3 | 4 |
      | 5 | 6 | 7 | 8 |
      | 9 | 8 | 7 | 6 |
      | 5 | 4 | 3 | 2 |
    And the following matrix B:
      | -2 | 1 | 2 |  3 |
      |  3 | 2 | 1 | -1 |
      |  4 | 3 | 6 |  5 |
      |  1 | 2 | 7 |  8 |
    Then A * B is the following 4x4 matrix:
      | 20 | 22 |  50 |  48 |
      | 44 | 54 | 114 | 108 |
      | 40 | 58 | 110 | 102 |
      | 16 | 26 |  46 |  42 |

  Scenario: A matrix multiplied by a tuple
    Given the following matrix A:
      | 1 | 2 | 3 | 4 |
      | 2 | 4 | 4 | 2 |
      | 8 | 6 | 4 | 1 |
      | 0 | 0 | 0 | 1 |
    And b ← tuple(1, 2, 3, 1)
    Then A * b = tuple(18, 24, 33, 1)

  Scenario: Multiplying a matrix by the identity matrix
    Given the following matrix A:
      | 0 | 1 |  2 |  4 |
      | 1 | 2 |  4 |  8 |
      | 2 | 4 |  8 | 16 |
      | 4 | 8 | 16 | 32 |
    Then A * identity_matrix = A

  Scenario: Multiplying the identity matrix by a tuple
    Given a ← tuple(1, 2, 3, 4)
    Then identity_matrix * a = a

  Scenario: Transposing a matrix
    Given the following matrix A:
      | 0 | 9 | 3 | 0 |
      | 9 | 8 | 0 | 8 |
      | 1 | 8 | 5 | 3 |
      | 0 | 0 | 5 | 8 |
    Then transpose(A) is the following matrix:
      | 0 | 9 | 1 | 0 |
      | 9 | 8 | 8 | 0 |
      | 3 | 0 | 5 | 5 |
      | 0 | 8 | 3 | 8 |

  Scenario: Transposing the identity matrix
    Given A ← transpose(identity_matrix)
    Then A = identity_matrix

  Scenario: Calculating the determinant of a 2x2 matrix
    Given the following 2x2 matrix A:
      |  1 | 5 |
      | -3 | 2 |
    Then determinant(A) = 17

  Scenario: A submatrix of a 3x3 matrix is a 2x2 matrix
    Given the following 3x3 matrix A:
      |  1 | 5 |  0 |
      | -3 | 2 |  7 |
      |  0 | 6 | -3 |
    Then submatrix(A, 0, 2) is the following 2x2 matrix:
      | -3 | 2 |
      |  0 | 6 |

  Scenario: A submatrix of a 4x4 matrix is a 3x3 matrix
    Given the following 4x4 matrix A:
      | -6 | 1 |  1 | 6 |
      | -8 | 5 |  8 | 6 |
      | -1 | 0 |  8 | 2 |
      | -7 | 1 | -1 | 1 |
    Then submatrix(A, 2, 1) is the following 3x3 matrix:
      | -6 |  1 | 6 |
      | -8 |  8 | 6 |
      | -7 | -1 | 1 |

  Scenario: Calculating a minor of a 3x3 matrix
    Given the following 3x3 matrix A:
      | 3 |  5 |  0 |
      | 2 | -1 | -7 |
      | 6 | -1 |  5 |
    And B ← submatrix(A, 1, 0)
    Then determinant(B) = 25
    And minor(A, 1, 0) = 25

  Scenario: Calculating a cofactor of a 3x3 matrix
    Given the following 3x3 matrix A:
      | 3 |  5 |  0 |
      | 2 | -1 | -7 |
      | 6 | -1 |  5 |
    Then minor(A, 0, 0) = -12
    And cofactor(A, 0, 0) = -12
    And minor(A, 1, 0) = 25
    And cofactor(A, 1, 0) = -25

  Scenario: Calculating the determinant of a 3x3 matrix
    Given the following 3x3 matrix A:
      |  1 | 2 |  6 |
      | -5 | 8 | -4 |
      |  2 | 6 |  4 |
    Then cofactor(A, 0, 0) = 56
    And cofactor(A, 0, 1) = 12
    And cofactor(A, 0, 2) = -46
    And determinant(A) = -196

  Scenario: Calculating the determinant of a 4x4 matrix
    Given the following 4x4 matrix A:
      | -2 | -8 |  3 |  5 |
      | -3 |  1 |  7 |  3 |
      |  1 |  2 | -9 |  6 |
      | -6 |  7 |  7 | -9 |
    Then cofactor(A, 0, 0) = 690
    And cofactor(A, 0, 1) = 447
    And cofactor(A, 0, 2) = 210
    And cofactor(A, 0, 3) = 51
    And determinant(A) = -4071

  Scenario: Testing an invertible matrix for invertibility
    Given the following 4x4 matrix A:
      | 6 |  4 | 4 |  4 |
      | 5 |  5 | 7 |  6 |
      | 4 | -9 | 3 | -7 |
      | 9 |  1 | 7 | -6 |
    Then determinant(A) = -2120
    And A is invertible

  Scenario: Testing a noninvertible matrix for invertibility
    Given the following 4x4 matrix A:
      | -4 |  2 | -2 | -3 |
      |  9 |  6 |  2 |  6 |
      |  0 | -5 |  1 | -5 |
      |  0 |  0 |  0 |  0 |
    Then determinant(A) = 0
    And A is not invertible

  Scenario: Calculating the inverse of a matrix
    Given the following 4x4 matrix A:
      | -5 |  2 |  6 | -8 |
      |  1 | -5 |  1 |  8 |
      |  7 |  7 | -6 | -7 |
      |  1 | -3 |  7 |  4 |
    And B ← inverse(A)
    Then determinant(A) = 532
    And cofactor(A, 2, 3) = -160
    And B[3,2] = -160/532
    And cofactor(A, 3, 2) = 105
    And B[2,3] = 105/532
    And B is the following 4x4 matrix:
      |  0.21804512  |  0.45112783 |  0.24060151 | -0.04511278 |
      | -0.8082707   | -1.456767   | -0.44360903 |  0.5206767  |
      | -0.078947365 | -0.2236842  | -0.05263158 |  0.19736843 |
      | -0.52255636  | -0.81390977 | -0.30075186 |  0.30639097 |

  Scenario: Calculating the inverse of another matrix
    Given the following 4x4 matrix A:
      |  8 | -5 |  9 |  2 |
      |  7 |  5 |  6 |  1 |
      | -6 |  0 |  9 |  6 |
      | -3 |  0 | -9 | -4 |
    Then inverse(A) is the following 4x4 matrix:
      | -0.15384616 | -0.15384616 | -0.2820513   | -0.53846157 |
      | -0.07692308 |  0.12307692 |  0.025641026 |  0.03076923 |
      |  0.35897437 |  0.35897437 |  0.43589744  |  0.9230769  |
      | -0.6923077  | -0.6923077  | -0.7692308   | -1.9230769  |

  Scenario: Calculating the inverse of a third matrix
    Given the following 4x4 matrix A:
      |  9 |  3 |  0 |  9 |
      | -5 | -2 | -6 | -3 |
      | -4 |  9 |  6 |  4 |
      | -7 |  6 |  6 |  2 |
    Then inverse(A) is the following 4x4 matrix:
      | -0.04074074  | -0.07777778  |  0.14444445 | -0.22222222 |
      | -0.07777778  |  0.033333335 |  0.36666667 | -0.33333334 |
      | -0.029012345 | -0.14629629  | -0.10925926 |  0.12962963 |
      |  0.17777778  |  0.06666667  | -0.26666668 |  0.33333334 |

  Scenario: Multiplying a product by its inverse
    Given the following 4x4 matrix A:
      |  3 | -9 |  7 |  3 |
      |  3 | -8 |  2 | -9 |
      | -4 |  4 |  4 |  1 |
      | -6 |  5 | -1 |  1 |
    And the following 4x4 matrix B:
      | 8 |  2 | 2 | 2 |
      | 3 | -1 | 7 | 0 |
      | 7 |  0 | 5 | 4 |
      | 6 | -2 | 0 | 5 |
    And C ← A * B
    Then C * inverse(B) = A
