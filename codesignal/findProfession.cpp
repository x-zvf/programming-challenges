string findProfession(int level, int pos) {
    if (level < 2)
        return "Engineer";
    if (findProfession(level-1, (pos+1)/2) == "Doctor")
        return pos%2 == 0 ? "Engineer" : "Doctor";
    return pos%2 == 0 ?  "Doctor" : "Engineer";
}

