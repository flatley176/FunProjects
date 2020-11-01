rm(list=ls())
library(ggplot2)
library(sf)
theme_set(theme_bw())
library("rnaturalearth")
library("rnaturalearthdata")
library(glue)
world <- ne_countries(scale = 110, returnclass = "sf")
class(world)
wd <- "/Users/sr/git_personal/FunProjects/trivia"
setwd(wd)
args = commandArgs(trailingOnly=TRUE)
if(length(args)!=1) {
    stop("places data frame needed", call.=FALSE)
}
data <- read.table(args[1], header=T)

questions <- unique(data$Question)

sapply(questions, function(q) {
	lt <- data[data$Question==q,]$Latitude
	lg <- data[data$Question==q,]$Longitude
	buffer <- 20
	sites <- data.frame(latitude = lt, longitude = lg)
	p <- ggplot(data = world) +
  		geom_sf() +
  		geom_point(data = sites, aes(x = longitude, y = latitude), size = 4, shape = 23, fill = "darkred") +
  		coord_sf(xlim = c(min(sites$longitude)-buffer,max(sites$longitude)+buffer), ylim = c(min(sites$latitude)-buffer, max(sites$latitude)+buffer), expand = FALSE)
	ggsave(paste("question_", q, ".png", sep=""), p, width=7, height=7, units="in")
})

