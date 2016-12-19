rm(list=ls())
library(rvest)
url <- "http://www.livesinabox.com/friends/scripts.shtml"
dir <- dirname(url)

##Scrape page and get all URLs
url_page <- read_html(url,  encoding = "ISO-8859-1")

all_urls <- url_page %>% 
  html_nodes("div") %>% 
  html_nodes("a") %>% 
  html_attr("href")
patterns=c("season", "^10")
s1_10 <- unique(paste(dir,all_urls %>%
                        grep(paste(patterns, collapse="|"), ., value=T), sep="/"))

##Function to extract scripts given the script url
getScript <- function(page_url){
  read_html(page_url, encoding = "ISO-8859-1") %>%
    html_nodes("p") %>%
    html_text() %>%
    gsub("\n"," ",.)
  }

##Extract scripts
library(parallel)
script_list <- mclapply(s1_10, mc.cores = detectCores(), getScript)

grep("Scene:", script_list[[1]])
