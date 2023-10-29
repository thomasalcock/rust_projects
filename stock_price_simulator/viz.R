library(ggplot2)
df <- read.csv("outputs/simulation.csv", header=T)
df$path_index <- as.character(df$path_index)

ggplot(df, aes(x = time_index, y = price, col = path_index)) + 
    geom_line() + theme_minimal() + 
    guides(color = "none") + 
    labs(x = "t", y = "S(t)")

ggsave("outputs/plot.png")