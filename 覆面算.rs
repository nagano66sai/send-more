//覆面算:send+more=moneyを解きます。s,e,n,d,m,o,r,yはすべて互に異なる数字であり,sとmは0であってはなりません。
fn  main(){
for  s in  0..10{
for  e in  0..10{
for  n in  0..10{
for  d in  0..10{
for  m in  0..10{
for  o in  0..10{
for  r in  0..10{
for  y in  0..10{
if  (1000*s+100*e+10*n+d)+(1000*m+100*o+10*r+e)==10000*m+1000*o+100*n+10*e+y  && s!=e  && s!=n && s!=d && s!=m && s!=o && s!=r && s!=y && e!=n && e!=d && e!=m  && e!=o && e!=r  && e!=y  && n!=d   && n!=m && n!=o && n!=r && n!=y && d!=m  && d!=o && d!=r && d!=y && m!=o && m!=r && m!=y  && o!=r && o!=y  && r!=y && s!=0  && m!=0{                     
println!("send+more=money::{}{}{}{}+{}{}{}{}={}{}{}{}{}",s,e,n,d,m,o,r,e,m,o,n,e,y);
}
}}}}}}}}}