using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class DissolveSphere : MonoBehaviour {

    Material mat;
    [Range(0,1)]
    public float Slider;
    public float ScrollSpeed=0;
    private float offset;

    private void Start() {
        mat = GetComponent<Renderer>().material;
        ScrollSpeed = Random.value / 100f + 0.01f;
        offset = Random.value;
    }

    private void Update() {
        mat.SetFloat("_DissolveAmount", Slider);
        mat.SetFloat("_Offset", (offset+ Time.time * ScrollSpeed) % 1f);
    }
}