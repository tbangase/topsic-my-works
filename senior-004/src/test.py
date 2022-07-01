n, k = map(int, input().split())

A = []
B = []
for _ in range(n):
    a, b = map(float, input().split())
    A.append(a)
    B.append(b)

dp = [[0] * (k + 1) for _ in range(n + 1)]

for i in range(n):
    for j in range(1, k + 1):
        dp[i + 1][j] = dp[i][j]
        for l in range(1, j + 1):
            if j - l >= 0:
                dp[i + 1][j] = max(1 - (1 - dp[i][j - l]) * (1 - A[i] * (1 - B[i] ** l)), dp[i + 1][j])
print(dp[-1][-1])
