import java.util.*;
public class Main {
    public static void main(String[] args) throws Exception {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        long xp = sc.nextLong();
        long yp = sc.nextLong();
        long xq = sc.nextLong();
        long yq = sc.nextLong();
        long[][] r2p = new long[n+1][2];
        long[][] r2q = new long[n+1][2];
        r2p[0][0] = 0;
        r2q[0][0] = 0;
        r2p[0][1] = 0;
        r2q[0][1] = 0;
        // ここまで初期化
        for(int i = 1; i < n+1; i++){
            long a = sc.nextLong();
            long b = sc.nextLong();
            r2p[i-1][0] = (xp-a)*(xp-a) + (yp-b)*(yp-b);
            r2p[i-1][1] = i;
            r2q[i-1][0] = (xq-a)*(xq-a) + (yq-b)*(yq-b);
            r2q[i-1][1] = i;
        }
        // pは昇順でソート
        Arrays.sort(r2p, (a, b) -> Long.compare(a[0], b[0]));
        // qは降順でソート
        Arrays.sort(r2q, (a, b) -> Long.compare(-a[0], -b[0]));

        // 集合
        // pの円に含める点の集合
        Set<Long> set = new HashSet<Long>();
        set.add((long)0);

        int st = 0;
        long ans = LongMAX_VALUE;
        for(long[] r2pi: r2p){
            // pの最小値を求める
            set.add(r2pi[1]);
            for(int i = st; i < n+1; i++){
                if(set.contains(r2q[i][1])){
                    // pの円に含まれていれば
                    st += 1;
                } else{
                    // pの円に含まれてなかったら
                    ans = Math.min(ans, r2pi[0]+r2q[i][0]);
                    break;
                }
            }
        }
        ans = Math.min(ans, r2p[n][0]);
        System.out.println(ans);
    }
}.
