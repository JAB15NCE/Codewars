#include <stddef.h>

size_t duplicate_count(const char *text) {
  int counts[36] = {0};
  size_t duplicates = 0;
  
  for (int i = 0; text[i] != '\0'; i++){
    char c = tolower(text[i]);
    if(isalpha(c)){
      counts[c - 'a']++;
    }else if(isdigit(c)){
      counts[c -'0' + 26]++;
    }
  }
  for (int i = 0; i < 36; i++){
    if (counts[i] > 1){
      duplicates++;
    }
  }
  
    return duplicates;
}
