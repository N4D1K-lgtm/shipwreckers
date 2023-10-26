# JSON Representation

```json
{
  "name": "DetailedMapNoise",
  "constants": {
    "land_frequency": 0.04,
    "base_seed": 1234,
    "ridge_multiplier": 1.5
  },
  "node_templates": {
    "land_template": {
      "type": "perlin",
      "parameters": {
        "frequency": "land_frequency",
        "seed": "base_seed"
      }
    },
    "ridge_template": {
      "type": "ridgedmulti",
      "parameters": {
        "multiplier": "ridge_multiplier",
        "seed": "base_seed"
      }
    }
  },
  "output": "final_map"
  "graph": {
    "land1": {
      "template": "land_template"
    },
    "ridge_instance": {
      "template": "ridge_template"
    },
    "scaled_ridge": {
      "type": "scale",
      "parameters": {
        "input": "ridge_instance",
        "scale": 0.8
      }
    },
    "water": {
      "type": "billow",
      "parameters": {
        "frequency": 0.02,
        "seed": 5678
      }
    },
    "combined_noise": {
      "type": "add",
      "parameters": {
        "input1": "land1",
        "input2": "scaled_ridge"
      }
    },
    "final_map": {
      "type": "blend",
      "parameters": {
        "input": "combined_noise",
        "other": "water",
        "control": "land1"
      }
    },
    "clamped_map": {
      "type": "clamp",
      "parameters": {
        "input": "final_map",
        "lower_bound": 0.0,
        "upper_bound": 1.0
      }
    }
  }
}
```

```

```
