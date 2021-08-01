std::vector<int> deleteNth(std::vector<int> arr, int n)
{
  std::map<int,int> counts;
  for(auto it = std::begin(arr); it != std::end(arr);){
    ++counts[*it];
    if(counts[*it] > n) it = arr.erase(it);
    else ++it;
  }
  return arr;
}