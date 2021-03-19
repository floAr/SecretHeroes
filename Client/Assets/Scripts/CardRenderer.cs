using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CardRenderer : MonoBehaviour
{
    // Start is called before the first frame update
    public CardData Data;

    public GameObject[] Models;
    private int _activeModel;


    [ContextMenu("Click")]
    public void MixModel()
    {
        SetModel(Random.Range(0, Models.Length));
    }

    private void SetModel(int newModel) {
        Models[_activeModel].SetActive(false);
        Models[newModel].SetActive(true);
        _activeModel = newModel;
    }



    void Start()
    {
        MixModel();
    }

    // Update is called once per frame
    void Update()
    {
        
    }
}
