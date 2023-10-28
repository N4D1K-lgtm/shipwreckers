# Example Noise Asset

````json
{
  "name": "TerrainNoiseMap",
  "constants": {
    "MAIN_FREQUENCY": {
      "float": 2.5
    },
    "MAIN_OCTAVES": {
      "usize": 5
    },
    "SCALE_FACTOR": {
      "float": 3.0
    },
    "TERRAIN_SEED": {
      "unsignedint": 1234567890
    }
  },
  "node_templates": {
    "ForestLayer": {
      "name": "ForestLayer",
      "inputs": [
        "base_noise"
      ],
      "output": "forest_ridged",
      "nodes": {
        "forest_ridged": {
          "RidgedMulti": {
            "input": "base_noise",
            "octaves": {
              "value": 6
            },
            "frequency": {
              "value": 2.0
            },
            "lacunarity": {
              "value": 2.0943951023931953
            },
            "persistence": {
              "value": 0.5
            }
          }
        }
      }
    },
    "MountainBase": {
      "name": "MountainBase",
      "inputs": [
        "base_noise",
        "mountain_exponent"
      ],
      "output": "mountain_power",
      "nodes": {
        "mountain_power": {
          "Power": {
            "base": "base_noise",
            "exponent": "mountain_exponent"
          }
        }
      }
    }
  },
  "output": "final_terrain",
  "graph": {
    "mountain_layer": {
      "template": "MountainBase",
      "inputs": {
        "base_noise": "base_noise"
      }
    },
    "base_noise": {
      "ImprovedPerlin": {
        "seed": {
          "constant": "TERRAIN_SEED"
        }
      }
    },
    "forest_layer": {
      "template": "ForestLayer",
      "inputs": {
        "base_noise": "base_noise",
        "forest_layer": "forest_layer"
      }
    },
    "final_terrain": {
      "Blend": {
        "base": "mountain_layer",
        "other": "forest_layer",
        "control": "base_noise"
      }
    }
  }
}```
````
