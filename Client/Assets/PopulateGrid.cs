using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class PopulateGrid : MonoBehaviour
{
    public GameObject prefab;
    public int numberToCreate;
    void Start()
    {
        Populate();
    }

    // Update is called once per frame
    void Update()
    {
        
    }
    void Populate()
    {
        GameObject newObj;

        for (int i = 0; i < numberToCreate; i++)
        {
            newObj = (GameObject)Instantiate(prefab, transform);
           // newObj.GetComponent<Image>().color = Random.ColorHSV();

        }
    }
}
