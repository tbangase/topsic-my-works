import java.util.*;
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int r = sc.nextInt();
        int c = sc.nextInt();
        char[][] grid = new char[r][c];
        for (int i = 0; i < r; i++) {
            grid[i] = sc.next().toCharArray();
        }
        int[][] directions = {{1, 0}, {0, 1}, {-1, 0}, {0, -1}};
        int[][] distances = new int[r][c];
        Queue<int[]> q = new ArrayDeque<int[]>();
        for (int i = 0; i < r; i++) {
            for (int j = 0; j < c; j++) {
                if (grid[i][j] == '*') {
                    q.add(new int[] {i, j});
                } else {
                    distances[i][j] = -1;
                }
            }
        }
       int ans = 0;
        while (!q.isEmpty()) {
            int[] p = q.poll();
            int y = p[0];
            int x = p[1];
            ans = Math.max(ans, distances[y][x]);
            for (int[] d : directions) {
                int ny = y+d[0];
                int nx = x+d[1];
                if (0 <= ny && ny < r && 0 <= nx && nx < c && distances[ny][nx] == -1) {
                    q.add(new int[] {ny, nx});
                    distances[ny][nx] = distances[y][x]+1;
                }
            }
        }
        System.out.println(ans);
    }
} 
