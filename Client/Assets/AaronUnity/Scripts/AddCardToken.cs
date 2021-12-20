using System;
using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;
using UnityEngine.UI;

public static class ButtonExtension
{
    // Call Custom extended event Listener ButtonExtension
    public static void addEventListener<T>(this Button button, T param, Action<T> OnClick)
    {
        button.onClick.AddListener(delegate ()
        {
            OnClick(param);
        });
    }
}
public class AddCardToken : MonoBehaviour
{
    #region Attributes
    public GameObject templateCard;
    GameObject g;
    [SerializeField] Transform CardsListUI;

    [SerializeField] GameObject imageHolder;
    [SerializeField] GameObject statsHolder;

    [SerializeField] GameObject OptionButtonsHolder;
    [SerializeField] GameObject UpgradeHeroesHolder;
    [SerializeField] GameObject OddsBurnHolder;

    [SerializeField] GameObject OddsBurnHolder_fieds;

    [SerializeField] GameObject btnCancel;

    [SerializeField] Sprite overrideImage;
    [SerializeField] TextMeshProUGUI TopTitle;

    private int i;
    private int selectedindex = 0;
    private string selectionState;

    private CardDisplay tempGameObject;

    public CardRenderer CharacterRenderer;

    private int randomUpgradeToken = 0;

    private List<string> selectedBoth;

    private string pathFolder = "";
    private string prefabName = "CardRenderer";

    public GameObject thumbnailCreator;

    public ParticleSystem fx;

    private int countSelectedToBurn = 0;

    private bool isNotFirsTime = false;

    private List<Token> runningTokens;

    #endregion

    #region Unity Methods
    void Awake()
    {
        Debug.Log("Awake.");
        StartCoroutine(InitializeGrid());
    }
    void Start()
    {
        Debug.Log("Grid has Started.");
        isNotFirsTime = true;
        runningTokens = new List<Token>();
        i = 0;
        selectionState = "selection_hero";
        selectedBoth = new List<string>();

        if (GameObject.FindObjectOfType<ThumbnailSetup>().selectCustomFolder)
            pathFolder = FindObjectOfType<ThumbnailSetup>().FolderFullPathName();
        else
            pathFolder = Application.dataPath;
    }
    void Update()
    {
        if (countSelectedToBurn == 1)
        {
            TopTitle.text = "Choose One More Hero To Burn";
        }

        else if (countSelectedToBurn == 2)
        {
            TopTitle.text = "Ready To Upgrade";
        }
    }
    void OnEnable()
    {
        Debug.Log("GridOnEnable.");
        countSelectedToBurn = 0;
        imageHolder.SetActive(true);
        statsHolder.SetActive(false);
        btnCancel.SetActive(false);
        UpgradeHeroesHolder.SetActive(false);
        OptionButtonsHolder.SetActive(false);
        if (isNotFirsTime) { StartCoroutine(RefreshGrid()); }
    }
    void OnDisable()
    {
        Debug.Log("Grid OnDisable.");
    }
    #endregion

    #region Custom Game Methods
    
    void onClickEvent(int index)
    {
        selectedindex = index;

        Debug.Log("Selected Hero Index: " + selectedindex);

        tempGameObject = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>();

        #region Check Selection State
        if (selectionState.Equals("selection_hero"))
        {
            imageHolder.SetActive(false);
            statsHolder.SetActive(true);
            btnCancel.SetActive(true);
            UpgradeHeroesHolder.SetActive(false);
            OptionButtonsHolder.SetActive(true);
            //AR Dec8
            selectionState.Equals("selection_to_burn");            
        }

        //AR Dec8 Added && countSelectedToBurn == 0
        else if (selectionState.Equals("selection_to_burn") && countSelectedToBurn == 0)
        {

            InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetComponent<Button>().interactable = false;

            InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetComponent<Button>().image.overrideSprite = overrideImage;

            InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetChild(2).GetComponent<Image>().enabled = true;

            ++countSelectedToBurn;
        }
        else if (selectionState.Equals("selection_to_burn") && countSelectedToBurn == 1)
        {
            InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetComponent<Button>().interactable = false;

            InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetComponent<Button>().image.overrideSprite = overrideImage;

            InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetChild(2).GetComponent<Image>().enabled = true;
            OddsBurnHolder.SetActive(true);

            ++countSelectedToBurn;
        }
        else if (selectionState.Equals("selection_to_burn") && countSelectedToBurn == 2)
        {
            return;
        }


        selectedBoth.Add(InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.GetComponent<CardDisplay>().txtId.text);

        #endregion

        if (!selectionState.Equals("selection_hero")) return;

        #region Update Thumbnail Stats
        statsHolder.transform.GetChild(1).GetChild(0).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>().txtStat1.text;

        statsHolder.transform.GetChild(1).GetChild(1).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>().txtStat2.text;

        statsHolder.transform.GetChild(1).GetChild(2).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>().txtStat3.text;

        statsHolder.transform.GetChild(1).GetChild(3).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>().txtStat4.text;

        statsHolder.transform.GetChild(1).GetChild(4).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>().txtName.text;

        statsHolder.transform.GetChild(0).GetComponentInChildren<Image>().sprite = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>().image.sprite;

        #endregion

    }

    private void updateNewStats(int a, int b, int c, int d)
    {
        #region update selected card

        statsHolder.transform.GetChild(1).GetChild(0).GetComponentInChildren<TextMeshProUGUI>().text = a.ToString();

        statsHolder.transform.GetChild(1).GetChild(1).GetComponentInChildren<TextMeshProUGUI>().text = b.ToString();

        statsHolder.transform.GetChild(1).GetChild(2).GetComponentInChildren<TextMeshProUGUI>().text = c.ToString();

        statsHolder.transform.GetChild(1).GetChild(3).GetComponentInChildren<TextMeshProUGUI>().text = d.ToString();

        #endregion
    }

    public void StartBattleAsTst()
    {
        Token token = GameObject.FindObjectOfType<Rooster>().FindToken(tempGameObject.txtId.text);
        if (token != null)
        {
            GameObject.FindObjectOfType<WebGlBridge>().TriggerBattle(token);
        }
    }

    public void onStartBothSelection()
    {
        if (InstantiatedTokens.instantiatedObjects.Count < 3) return;

        selectionState = "selection_to_burn";
        TopTitle.text = "Choose Two Heroes To Burn";

        OptionButtonsHolder.SetActive(false);
        //UpgradeHeroesHolder.SetActive(true);

        InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetComponent<Button>().image.overrideSprite = overrideImage;

        InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetComponent<Button>().interactable = false;
    }

    public void OnCancelButtonPressed()
    {
        // Unselect Cards & Refresh Grid Again
        StartCoroutine(RefreshGrid());
    }

    public void onCloseOddBurnPanel()
    {
        // Unselect Cards (Standard View)
        UnselectCardsForStandardView();

        // Close Upgrade Prob Popup
        OddsBurnHolder.SetActive(false);        
    }

    public void upgradeNow()
    {
        OddsBurnHolder.SetActive(false);

        // Get Random Stats
        var randomUpgradeToken = UnityEngine.Random.Range(0, 3);
        Debug.Log("randomUpgradeToken: " + randomUpgradeToken);

        #region Get new Values

        if (randomUpgradeToken == 0)
        {
            InstantiatedTokens.cardWeapons = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(4).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardEngineering = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(5).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardBiotech = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(6).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardPsychics = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(7).GetComponent<TextMeshProUGUI>().text);
        }
        else if (randomUpgradeToken == 1)
        {
            InstantiatedTokens.cardWeapons = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(8).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardEngineering = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(9).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardBiotech = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(10).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardPsychics = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(11).GetComponent<TextMeshProUGUI>().text);
        }
        else if (randomUpgradeToken == 2)
        {
            InstantiatedTokens.cardWeapons = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(12).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardEngineering = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(13).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardBiotech = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(14).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardPsychics = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(15).GetComponent<TextMeshProUGUI>().text);
        }

        #endregion

        // Burn & Remove them from List
        Debug.Log("selectedBoth: " + selectedBoth.Count);
        for (int i = 0; i < selectedBoth.Count; i++)
        {
            Token token1 = new Token()
            {
                id = selectedBoth[i]
            };

            Debug.Log("BurnedId: " + selectedBoth[i]);

            if (runningTokens.Count > 0)
            {
                GameObject.FindObjectOfType<Rooster>().RemoveToken(token1);
                runningTokens.Remove(token1);
            }
        }

        selectedBoth.Clear();

        updateNewStats(InstantiatedTokens.cardWeapons,
                InstantiatedTokens.cardEngineering,
                InstantiatedTokens.cardBiotech,
                InstantiatedTokens.cardPsychics);

        fx.Play();

        Token token = new Token()
        {
            id = tempGameObject.txtId.text,
            weapons = InstantiatedTokens.cardWeapons,
            engineering = InstantiatedTokens.cardEngineering,
            biotech = InstantiatedTokens.cardBiotech,
            psychics = InstantiatedTokens.cardPsychics

        };
        GameObject.FindObjectOfType<Rooster>().UpdateToken(token);

        // Initialize GridView  
        i = 0;
        selectionState = "selection_hero";
        selectedBoth = new List<string>();
        imageHolder.SetActive(false);
        statsHolder.SetActive(true);
        btnCancel.SetActive(true);
        UpgradeHeroesHolder.SetActive(false);
        OptionButtonsHolder.SetActive(false);
        TopTitle.text = "Choose Hero";

        for (int k = 0; k < CardsListUI.transform.childCount; k++)
            Destroy(CardsListUI.transform.GetChild(k).gameObject);

        InstantiatedTokens.instantiatedObjects.Clear();

        thumbnailCreator.SetActive(true);

        int count = GameObject.FindObjectOfType<Rooster>().MyHeroes.Count;
        if (count > 0)
        {
            foreach (Token currentToken in GameObject.FindObjectOfType<Rooster>().MyHeroes)
            {
                StartCoroutine(readNewCardToken(currentToken));
            }
        }

        thumbnailCreator.SetActive(false);
    }

    public void onOpenOddBurnPanel()
    {
        OddsBurnHolder.SetActive(true);

        #region Fill Selected Main Card
        OddsBurnHolder_fieds.transform.GetChild(0).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(1).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(2).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(3).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion


        #region Fill Green 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(4).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(5).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(6).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(7).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion

        #region Fill White 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(8).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(9).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(10).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(11).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion

        #region Fill Red 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(12).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(13).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(14).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(15).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion


        #region Fill Small Green 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(16).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(17).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(18).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(19).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion

        #region Fill Small White 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(20).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(21).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(22).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(23).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion

        #region Fill Small Red 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(24).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(25).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(26).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(27).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion


        #region Fill Extra Small Green 33% Burned Card
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(28).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(29).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(30).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(31).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));

        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(28).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(29).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(30).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(31).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));

        updateOldField(OddsBurnHolder_fieds.transform.GetChild(4).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(28).GetComponent<TextMeshProUGUI>().text));
        updateOldField(OddsBurnHolder_fieds.transform.GetChild(5).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(29).GetComponent<TextMeshProUGUI>().text));
        updateOldField(OddsBurnHolder_fieds.transform.GetChild(6).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(30).GetComponent<TextMeshProUGUI>().text)); updateOldField(OddsBurnHolder_fieds.transform.GetChild(7).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(31).GetComponent<TextMeshProUGUI>().text));

        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(4).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(28).GetComponent<TextMeshProUGUI>().text));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(5).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(29).GetComponent<TextMeshProUGUI>().text));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(6).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(30).GetComponent<TextMeshProUGUI>().text));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(7).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(31).GetComponent<TextMeshProUGUI>().text));

        #endregion

        #region Fill Extra Small White 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(32).GetComponent<TextMeshProUGUI>().text = "+0";
        OddsBurnHolder_fieds.transform.GetChild(33).GetComponent<TextMeshProUGUI>().text = "+0";
        OddsBurnHolder_fieds.transform.GetChild(34).GetComponent<TextMeshProUGUI>().text = "+0";
        OddsBurnHolder_fieds.transform.GetChild(35).GetComponent<TextMeshProUGUI>().text = "+0";
        #endregion

        #region Fill Extra Small Red 33% Burned Card

        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(36).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(37).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(38).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(39).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));

        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(36).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(37).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(38).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(39).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));

        updateOldField(OddsBurnHolder_fieds.transform.GetChild(12).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(36).GetComponent<TextMeshProUGUI>().text));
        updateOldField(OddsBurnHolder_fieds.transform.GetChild(13).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(37).GetComponent<TextMeshProUGUI>().text));
        updateOldField(OddsBurnHolder_fieds.transform.GetChild(14).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(38).GetComponent<TextMeshProUGUI>().text)); updateOldField(OddsBurnHolder_fieds.transform.GetChild(15).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(39).GetComponent<TextMeshProUGUI>().text));

        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(12).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(36).GetComponent<TextMeshProUGUI>().text));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(13).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(37).GetComponent<TextMeshProUGUI>().text));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(14).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(38).GetComponent<TextMeshProUGUI>().text));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(15).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(39).GetComponent<TextMeshProUGUI>().text));


        #endregion


        #region Fill Card Name
        OddsBurnHolder_fieds.transform.GetChild(40).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtName.text;
        #endregion

    }

    private void printIndex(int index)
    {
        // Debug.Log("selected index: " + index);
    }

    private void setColorOfFiled(TextMeshProUGUI entity, int value)
    {
        if (value > 0) entity.color = new Color(0, 1, 0);
        else if (value < 0) entity.color = new Color(1, 0, 0);
        else entity.color = new Color(1, 1, 1);
    }

    private void setSignOfFiled(TextMeshProUGUI entity, int value)
    {
        if (value > 0) entity.text = "+" + value;
        else if (value == 0) entity.text = "+" + value;
        else entity.text = "" + value;
    }

    private void updateOldField(TextMeshProUGUI entity, int value)
    {
        entity.text = (Convert.ToInt16(entity.text) + value).ToString();
    }

    private IEnumerator InitializeGrid()
    {
        Debug.Log("InitializeGrid");

        i = 0;
        selectionState = "selection_hero";
        selectedBoth = new List<string>();
        imageHolder.SetActive(true);
        statsHolder.SetActive(false);
        btnCancel.SetActive(false);
        UpgradeHeroesHolder.SetActive(false);
        OptionButtonsHolder.SetActive(false);
        TopTitle.text = "Choose Hero";

        for (int k = 0; k < CardsListUI.transform.childCount; k++)
            Destroy(CardsListUI.transform.GetChild(k).gameObject);

        yield return new WaitForSeconds(1f);

        int count = GameObject.FindObjectOfType<Rooster>().MyHeroes.Count;
        if (count > 0)
        {
            thumbnailCreator.SetActive(true);

            Debug.Log("count: " + count);
            foreach (Token currentToken in GameObject.FindObjectOfType<Rooster>().MyHeroes)
            {
                runningTokens.Add(currentToken);
                StartCoroutine(readNewCardToken(currentToken));
                Debug.Log("runningTokens: " + runningTokens.Count);
            }
        }

        thumbnailCreator.SetActive(false);

        yield return true;
    }

    private IEnumerator readNewCardToken(Token currentToken)
    {
        g = Instantiate(templateCard, CardsListUI);

        CharacterRenderer.ReadToken(currentToken);

        GameObject.FindObjectOfType<ThumbnailSetup>().executeThumbnailCapture();

        g.transform.gameObject.GetComponent<CardDisplay>().updateTokenValues(currentToken.name, currentToken.weapons, currentToken.engineering, currentToken.biotech, currentToken.psychics, currentToken.id, pathFolder + "/" + prefabName + ".png");

        g.transform.GetChild(0).GetComponent<Button>().addEventListener(i, onClickEvent);

        InstantiatedTokens.instantiatedObjects.Add(g);

        i++;

        yield return new WaitForSeconds(1.75f);
    }

    private IEnumerator RefreshGrid()
    {
        Debug.Log("RefreshGrid");
        runningTokens.Clear();
        int count = GameObject.FindObjectOfType<Rooster>().MyHeroes.Count;
        if (count > 0)
        {
            thumbnailCreator.SetActive(true);

            Debug.Log("new count: " + count);
            foreach (Token currentToken in GameObject.FindObjectOfType<Rooster>().MyHeroes)
            {
                if (runningTokens.Count > 0 && runningTokens.Find(t => t.id == currentToken.id) == null)
                {
                    runningTokens.Add(currentToken);
                    StartCoroutine(readNewCardToken(currentToken));
                }
            }
        }

        thumbnailCreator.SetActive(false);

        yield return true;
    }

    private void UnselectCardsForStandardView() {
        
        for (int k = 0; k < CardsListUI.transform.childCount; k++) {
            InstantiatedTokens.instantiatedObjects[k].gameObject.transform.GetChild(0).GetComponent<Button>().interactable = true;

            InstantiatedTokens.instantiatedObjects[k].gameObject.transform.GetChild(0).GetChild(2).GetComponent<Image>().enabled = false;
        }

        Debug.Log("InitializeAgainGrid");
        selectionState = "selection_hero";
        selectedBoth = new List<string>();
        imageHolder.SetActive(true);
        statsHolder.SetActive(false);
        btnCancel.SetActive(false);
        UpgradeHeroesHolder.SetActive(false);
        OptionButtonsHolder.SetActive(false);
        TopTitle.text = "Choose Hero";
        countSelectedToBurn= 0;

        StartCoroutine(RefreshGrid());
    }

    #endregion
}