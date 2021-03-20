using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Drone : MonoBehaviour
{

    public GameObject DronePrefab;

    public GameObject DroneChild;

    public GameObject[] Path;

    public float LerpTime;

    public LeanTweenType TweenType = LeanTweenType.notUsed;

    LTDescr _current;
    int _id;

    // Start is called before the first frame update
    void Start()
    {

    }

    [ContextMenu("Lets go")]
    void LetsGo()
    {
        DroneChild = GameObject.Instantiate(DronePrefab, Path[0].transform.position, Quaternion.identity);
            _id = 0;
    }


    private void LerpToTransform(Transform toTrans)
    {
        _current = LeanTween.move(DroneChild.gameObject, toTrans.position, LerpTime).setEase(TweenType);
        LeanTween.rotate(DroneChild.gameObject, toTrans.rotation.eulerAngles, LerpTime).setEase(TweenType);

    }


    // Update is called once per frame
    void Update()
    {
        if(DroneChild!= null && _current == null)
        {
            _id += 1;
            if (_id == Path.Length)
            {
                GameObject.Destroy(DroneChild);
                return;
            }
          
            LerpToTransform(Path[_id].transform);
        }
        if (_current != null)
            if (_current.ratioPassed > 0.99)
            {
                _current = null;
            }

    }
}
