# Decimation Benchmark

## Purpose

This decimation benchmark will compare in a deterministic and automated manner several decimation algorithm implementation. This is for research purpose, in an attempt to improve ou current approaches to mesh optimization.

## Methodology

This is a benchmark application that consists in a PBR renderer and gradually decimated models from several sources (Blender, Nanomesh, ...)  
For each model, renders of the models are taken under various angles and perceptual hashes are computed and combined together, to finally estimate the perceptual distance from the original model.  
From this data it is now possible to plot the perceptual fidelity / the polygon reduction and find out which decimation algorithms yield the best results on each model.
