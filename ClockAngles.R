rm(list=ls())
computeAngle <- function(h1, m) {
  if(h1==12) {h=0} else {h=h1}
  hour_degrees=h*30
  minute_degrees=m*6
  h_difference=minute_degrees/12
  angle=abs(hour_degrees+h_difference-minute_degrees)
  c(h1,m,angle)
}

m_range <- rep(seq(0,59,1),12)
h_range <- sort(rep(seq(0,11,1),60))

results <- as.data.frame(t((mapply(computeAngle, h_range, m_range))))
colnames(results) <- c("hour", "minute", "angle")
attach(results)
results_ordered <- results[order(hour,minute),]
detach(results)
head(results_ordered)
results_ordered$time <- paste(sprintf("%02d",results_ordered$hour), sprintf("%02d",results_ordered$minute), sep=":")
library(ggplot2)
breaks_x<-results_ordered$time[seq(0,660,60)+1]
breaks_y<-seq(30,360,30)
breaks_x_min <- results_ordered$time[which(results_ordered$angle>0 & results_ordered$angle<3)]

blue.bold.text <- element_text(face = "bold", color = "blue", size = 10)
ggplot(results_ordered, aes(x=time, y=angle, group=1)) + 
  geom_line(colour="red", size=1) + 
  scale_x_discrete(breaks=breaks_x_min) + 
  scale_y_continuous(breaks=breaks_y) +
  theme_bw() + 
  labs(x = "Time", y = "Angle (°C)") + 
  theme(axis.text.x = blue.bold.text, axis.text.y=blue.bold.text)


## axis.text.x for x axis only
