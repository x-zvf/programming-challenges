#include <bits/stdc++.h>
using namespace std;


int main()
{
    cin.tie(0);
    cout.tie(0);
    ios_base::sync_with_stdio(false);

    int64_t n, m;
    cin >> n >> m;

    for(int i = 0; i < n; i++) {
        int64_t a, b, c;
        cin >> a >> b >> c;

    }

    return 0;
}

//
void read(int &k)
{
	int x=0,f=1;char ch=getchar();
	while(ch<'0'||ch>'9') {if(ch=='-') f=-1;ch=getchar();}
	while(ch>='0'&&ch<='9') {x=x*10+ch-'0';ch=getchar();}
	k=x*f;
}  
 
int n,m,nn,ans;
int p[N],flag[N]; 
vector<int>ve[N];
ll dis[N];
struct edge
{
	int a,b,nt,w;
}e[N*2];
 
void anode(int x,int y,int w)
{
	nn++;e[nn].a=x;e[nn].b=y;e[nn].w=w;e[nn].nt=p[x];p[x]=nn;
	nn++;e[nn].a=y;e[nn].b=x;e[nn].w=w;e[nn].nt=p[y];p[y]=nn;
}
 
int findf(int x,int val)
{
	int l=0,r=ve[x].size()-1;
    while(l<=r)
	{
        int mid=(l+r)/2;
        if(ve[x][mid]==val) return mid;
        else if(ve[x][mid]<val) l=mid+1;
        else r=mid-1;
    }
	return -1;
}
 
void spfa(int x)
{
	queue<int>q;
	for(int i=1;i<=n;i++)
		dis[i]=1e18,flag[i]=0;
	dis[x]=0,flag[x]=1;
	q.push(1);
	while(!q.empty())
	{
		int k=q.front();q.pop();
		flag[k]=0;
		int tmp=dis[k];
		if(ve[k].size()>0)
		{
			int tmppos=findf(k,tmp);
			if(tmppos!=-1)
			{
                tmp++;
                tmppos++;
                while(tmppos<ve[k].size()&&tmp==ve[k][tmppos])
					tmp++,tmppos++;
            }
		}
		for(int i=p[k];i;i=e[i].nt)
		{
			int v=e[i].b;
			if(dis[v]>tmp+e[i].w)
			{
				dis[v]=tmp+e[i].w;
				if(!flag[v])
				{
					flag[v]=1;
					q.push(v);
				}
			}
		}
	}
}
 
int main()
{
	//freopen("D:\\code\\in.txt","r",stdin);
	read(n),read(m);
	for(int x,y,z,i=1;i<=m;i++)
	{
		read(x),read(y),read(z);
		anode(x,y,z); 
	}
	for(int num,i=1;i<=n;i++)
	{
		 read(num);
		 while(num--)
		 {
		 	int x;read(x);
		 	ve[i].push_back(x);
		 }
	}
	//for(int i=1;i<=n;i++){for(int j=0;j<ve[i].size();j++)printf("%d ",ve[i][j]);printf("\n");}
	spfa(1);
	if(dis[n]==1e18) ans=-1;
	else ans=dis[n];
	printf("%d\n",ans);
	return 0;
}