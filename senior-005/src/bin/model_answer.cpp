#define DEB
#include<bits/stdc++.h>
#define REP(i,m) for(int i=0;i<(m);++i)
#define REPN(i,m,in) for(int i=(in);i<(m);++i)
#define ALL(t) (t).begin(),(t).end()
#define CLR(a) memset((a),0,sizeof(a))
#define pb push_back
#define mp make_pair
#define fr first
#define sc second
using namespace std;
#ifdef DEB
#define dump(x)  cerr << #x << " = " << (x) << endl
#define prl cerr<<"called:"<< __LINE__<<endl
template<class T> void 
debug(T a,T b){ for(;a!=b;++a) cerr<<*a<<' ';cerr<<endl;}
#else
#define dump(x) ;
#define prl ;
template<class T> void debug(T a,T b){ ;}
#endif
template<class T> void chmin(T& a,const T& b) { if(a>b) a=b; }
template<class T> void chmax(T& a,const T& b) { if(a<b) a=b; }
typedef long long int lint;
typedef pair<int,int> pi;
namespace std{
  template<class S,class T>
  ostream &operator <<(ostream& out,const pair<S,T>& a){
    out<<'('<<a.fr<<','<<a.sc<<')';
    return out;
  }
}
const int INF=5e8;
int h,w;
char buf[505][505];
int cost[505][505];
int dx[]={0,1,0,-1},dy[]={1,0,-1,0};
int main(){
  cin>>h>>w;
  queue<pi> q;
  memset(cost,-1,sizeof(cost));
  REP(i,h){
    scanf("%s",buf[i]);
    REP(j,w){
      if(buf[i][j]=='*'){
        cost[i][j]=0;
        q.push({i,j});
      }
    }
  }
  while(!q.empty()){
    pi cur=q.front();q.pop();
    REP(d,4){
      int px=cur.sc+dx[d],py=cur.fr+dy[d];
      if(px>=0 && py>=0 && px<w && py<h && cost[py][px]==-1){
        cost[py][px]=cost[cur.fr][cur.sc]+1;
        q.push({py,px});
      }
    }
  }
  int res=0;
  REP(i,h) REP(j,w) chmax(res,cost[i][j]);
  cout<<res<<endl;
  return 0;
}
