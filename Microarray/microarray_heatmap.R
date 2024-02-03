library(dplyr)
library(BiocManager)
library(genefilter)
library(gplots)
library(RColorBrewer)
library(ALL)

data(ALL)

#function to calculate distance by using pearson correlation
dist_cor <- function(x) 
  {
    as.dist(1 - cor(t(x), method = "pearson"))
  }

#function for clustering
clus_wd2 <- function(x) 
  {
    hclust(x, method = "ward.D2")
  }


# getting samples with NEG and BCR/ABL translocation
neg_bcrabl <- as.numeric(ALL$mol.biol %>% c("NEG", "BCR/ABL"))
neg_bcrabl <- na.omit(neg_bcrabl)

#getting a logical vector of cancers originating from B cells
bcells <- grepl("^B", ALL$BT)

#sub-setting the data
dataset <- ALL[, neg_bcrabl & bcells]

#releveling of factors in the subsetted dataset
dataset$mol.biol <- droplevels(dataset$mol.biol)
dataset$mol.biol <- relevel(dataset$mol.biol, "NEG")

#finding the most variable gene in the dataset 
#by measuring standard deviation
dataset_sd <- rowSds(exprs(dataset))

top100 <- names(sort(dataset_sd, decreasing = TRUE))[1:100]
dataset_var <- dataset[top100,]

#color scheme for the heatmap
redblackgreen <- colorRampPalette(c("green", "black", "red"))(n = 100)

#setting labels for the classes
class_labels <- ifelse(dataset_var$mol.biol == "NEG", "grey70", "grey20")

heatmap.2(
  exprs(dataset_var), 
  distfun = dist_cor, 
  hclust = clus_wd2, 
  scale = "row", 
  col = redblackgreen, 
  labRow = "", 
  ColSideColors = class_labels, 
  trace = "none",
  density.info = "none"
)
